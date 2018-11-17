use chrono::Utc;
use diesel::{insert_into, prelude::*, update};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::super::{crypto::Encryptor, errors::Result, orm::Connection};

pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Binary,
        salt -> Nullable<Binary>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

impl super::SettingDao for Connection {
    fn get<K: Serialize, V: DeserializeOwned, E: Encryptor>(&self, e: &E, k: &K) -> Result<V> {
        let key = serde_json::to_string(k)?;
        let (val, salt) = settings::dsl::settings
            .select((settings::dsl::value, settings::dsl::salt))
            .filter(settings::dsl::key.eq(&key))
            .first::<(Vec<u8>, Option<Vec<u8>>)>(self)?;

        let val = match salt {
            Some(salt) => e.decrypt(&val, &salt)?,
            None => val,
        };
        Ok(serde_json::from_slice(val.as_slice())?)
    }

    fn set<K: Serialize, V: Serialize, E: Encryptor>(
        &self,
        e: &E,
        k: &K,
        v: &V,
        f: bool,
    ) -> Result<()> {
        let key = serde_json::to_string(k)?;
        let buf = serde_json::to_vec(v)?;

        let (val, salt) = if f {
            let (val, salt) = e.encrypt(&buf);
            (val, Some(salt))
        } else {
            (buf, None)
        };

        let now = Utc::now().naive_utc();

        match settings::dsl::settings
            .select(settings::dsl::id)
            .filter(settings::dsl::key.eq(&key))
            .first::<i32>(self)
        {
            Ok(id) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&id));
                update(it)
                    .set((
                        settings::dsl::value.eq(&val),
                        settings::dsl::salt.eq(&salt),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(settings::dsl::settings)
                    .values((
                        settings::dsl::key.eq(&key),
                        settings::dsl::value.eq(&val),
                        settings::dsl::salt.eq(&salt),
                        settings::dsl::updated_at.eq(&now),
                        settings::dsl::created_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        };
        Ok(())
    }
}
