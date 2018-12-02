#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{connection::SimpleConnection, delete, insert_into, prelude::*, update};

use super::super::{errors::Result, rfc::RFC822};
use super::Connection;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use self::schema::schema_migrations;

#[derive(Queryable)]
pub struct Item {
    #[cfg(feature = "sqlite")]
    pub id: i32,
    #[cfg(any(feature = "postgresql", feature = "mysql"))]
    pub id: i64,
    pub version: String,
    pub name: String,
    pub up: String,
    pub down: String,
    pub run_at: Option<NaiveDateTime>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:23} {:32} {}",
            self.version,
            self.name,
            match self.run_at {
                Some(v) => v.to_rfc822(),
                None => "N/A".to_string(),
            }
        )
    }
}

#[derive(Insertable)]
#[table_name = "schema_migrations"]
pub struct New<'a> {
    pub version: &'a str,
    pub name: &'a str,
    pub up: &'a str,
    pub down: &'a str,
}

impl<'a> fmt::Display for New<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.version, self.name)
    }
}

pub trait Migration {
    fn check(&self, items: &Vec<New>) -> Result<()>;
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<Item>>;
}

impl Migration for Connection {
    fn check(&self, items: &Vec<New>) -> Result<()> {
        self.batch_execute(UP)?;
        for it in items {
            let c: i64 = schema_migrations::dsl::schema_migrations
                .filter(schema_migrations::dsl::version.eq(it.version))
                .filter(schema_migrations::dsl::name.eq(it.name))
                .count()
                .get_result(self)?;
            if c == 0 {
                info!("migration {} not exist, insert it", it);
                insert_into(schema_migrations::dsl::schema_migrations)
                    .values(it)
                    .execute(self)?;
            }
        }
        Ok(())
    }
    fn migrate(&self) -> Result<()> {
        for it in self.versions()? {
            info!("find migration {}", it);
            if None == it.run_at {
                info!("migrate {}", it.up);
                self.batch_execute(&it.up)?;
                update(
                    schema_migrations::dsl::schema_migrations
                        .filter(schema_migrations::dsl::id.eq(it.id)),
                )
                .set(schema_migrations::dsl::run_at.eq(&Some(Utc::now().naive_utc())))
                .execute(self)?;
            }
        }
        Ok(())
    }
    fn rollback(&self) -> Result<()> {
        for it in self.versions()? {
            info!("find migration {}", it);
            if let Some(_) = it.run_at {
                info!("rollback {}", it.down);
                self.batch_execute(&it.down)?;
                delete(
                    schema_migrations::dsl::schema_migrations
                        .filter(schema_migrations::dsl::id.eq(it.id)),
                )
                .execute(self)?;
                return Ok(());
            }
        }
        warn!("database is empty");
        Ok(())
    }
    fn versions(&self) -> Result<Vec<Item>> {
        let items = schema_migrations::dsl::schema_migrations
            .order(schema_migrations::dsl::version.desc())
            .load(self)?;
        Ok(items)
    }
}
