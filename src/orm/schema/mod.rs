#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

use chrono::NaiveDateTime;

use super::super::errors::Result;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub version: String,
    pub name: String,
    pub up: String,
    pub down: String,
}

pub trait Migration {
    fn up(&self, version: &String, name: &String, script: &String) -> Result<()>;
    fn down(&self, version: &String, name: &String, script: &String) -> Result<()>;
    fn status(&self) -> Result<Vec<(String, String, Option<NaiveDateTime>)>>;
}
