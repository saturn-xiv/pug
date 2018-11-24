use chrono::NaiveDateTime;
use diesel::{connection::SimpleConnection, delete, insert_into, prelude::*};

use super::super::super::errors::Result;
use super::super::Connection;

pub const UP: &'static str = include_str!("up.sql");

table! {
    schema_migrations (version) {
        id -> Bigint,
        version -> Varchar,
        name -> Varchar,
        created_at -> Datetime,
    }
}

impl super::Migration for Connection {
    fn up(&self, version: &String, name: &String, script: &String) -> Result<()> {
        self.batch_execute(script)?;
        insert_into(schema_migrations::dsl::schema_migrations)
            .values((
                schema_migrations::dsl::version.eq(version),
                schema_migrations::dsl::name.eq(name),
            )).execute(self)?;
        Ok(())
    }
    fn down(&self, version: &String, name: &String, script: &String) -> Result<()> {
        self.batch_execute(script)?;
        delete(
            schema_migrations::dsl::schema_migrations
                .filter(schema_migrations::dsl::version.eq(version))
                .filter(schema_migrations::dsl::name.eq(name)),
        ).execute(self)?;
        Ok(())
    }
    fn status(&self) -> Result<Vec<(String, String, NaiveDateTime)>> {
        let items = schema_migrations::dsl::schema_migrations
            .select((
                schema_migrations::dsl::version,
                schema_migrations::dsl::name,
                schema_migrations::dsl::created_at,
            )).order(schema_migrations::dsl::created_at.desc())
            .load::<(String, String, NaiveDateTime)>(self)?;
        Ok(items)
    }
    fn exists(&self, version: &String, name: &String) -> Result<bool> {
        self.batch_execute(UP)?;
        let c: i64 = schema_migrations::dsl::schema_migrations
            .filter(schema_migrations::dsl::name.eq(name))
            .filter(schema_migrations::dsl::version.eq(version))
            .count()
            .get_result(self)?;
        Ok(c > 0)
    }
}
