use std::ops::Deref;

use chrono::Duration;
use r2d2_redis::redis::{cmd, Connection};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::super::errors::Result;

impl super::Cache for Connection {
    fn get<K, V, F>(&self, key: &String, ttl: Duration, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize,
    {
        let key = format!("cache://{}", key);
        let db = self.deref();
        match cmd("get").arg(&key).query::<Vec<u8>>(db) {
            Ok(buf) => {
                let val = serde_json::from_slice(buf.as_slice())?;
                Ok(val)
            }
            Err(_) => {
                error!("can't get from cache {:?}", key);
                let val = fun()?;
                let _: String = cmd("set")
                    .arg(&key)
                    .arg(serde_json::to_vec(&val)?.as_slice())
                    .arg("ex")
                    .arg(ttl.num_seconds())
                    .query(db)?;
                Ok(val)
            }
        }
    }
}
