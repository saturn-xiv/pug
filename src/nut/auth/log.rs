use std::net::IpAddr;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};

use super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::schema::logs;

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub ip: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "logs"]
pub struct New<'a> {
    pub user_id: &'a ID,
    pub ip: &'a str,
    pub message: &'a str,
}

pub trait Dao {
    fn add<S: Into<String>>(&self, user: &ID, ip: &IpAddr, message: S) -> Result<()>;
    fn all(&self, user: &ID, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add<S: Into<String>>(&self, user: &ID, ip: &IpAddr, message: S) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values(&New {
                user_id: user,
                ip: &ip.to_string(),
                message: &message.into(),
            })
            .execute(self)?;
        Ok(())
    }

    fn all(&self, user: &ID, limit: i64) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
}
