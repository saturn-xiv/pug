use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, update};

use super::super::super::errors::Result;
use super::super::Connection;
use super::Migration;

table! {
    schema_migrations (version) {
        id -> Integer,
        version -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

fn check_table(db: &Connection) -> Result<()> {
    Ok(())
}

fn created_at(db: &Connection, version: &String, name: &String) -> Option<NaiveDateTime> {
    match schema_migrations::dsl::schema_migrations
        .select(schema_migrations::dsl::created_at)
        .filter(schema_migrations::dsl::version.eq(version))
        .filter(schema_migrations::dsl::name.eq(name))
        .first::<NaiveDateTime>(db)
    {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

impl Migration for Connection {
    fn up(&self, version: &String, name: &String, script: &String) -> Result<()> {
        Ok(())
    }
    fn down(&self, version: &String, name: &String, script: &String) -> Result<()> {
        Ok(())
    }
    fn status(&self) -> Result<Vec<(String, String, Option<NaiveDateTime>)>> {
        Ok(Vec::new())
    }
}
