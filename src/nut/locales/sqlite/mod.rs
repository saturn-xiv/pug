use chrono::Utc;
use diesel::{insert_into, prelude::*, update};

use super::super::super::{errors::Result, orm::Connection};

pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    locales (id) {
        id -> Integer,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

impl super::LocaleDao for Connection {
    fn languages(&self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .load::<String>(self)?)
    }

    fn get(&self, lang: &String, code: &String) -> Result<String> {
        let msg = locales::dsl::locales
            .select(locales::dsl::message)
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<String>(self)?;
        Ok(msg)
    }
    fn set(&self, lang: &String, code: &String, message: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        match locales::dsl::locales
            .select(locales::dsl::id)
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<i32>(self)
        {
            Ok(id) => {
                let it = locales::dsl::locales.filter(locales::dsl::id.eq(&id));
                update(it)
                    .set((
                        locales::dsl::message.eq(message),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(locales::dsl::locales)
                    .values((
                        locales::dsl::lang.eq(lang),
                        locales::dsl::code.eq(code),
                        locales::dsl::message.eq(message),
                        locales::dsl::created_at.eq(&now),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        }
        Ok(())
    }
}
