#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

use chrono::NaiveDateTime;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use super::super::errors::Result;

#[derive(Queryable, Serialize)]
pub struct Locale {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait LocaleDao {
    fn languages(&self) -> Result<Vec<String>>;
    fn get(&self, lang: &String, code: &String) -> Result<String>;
    fn set(&self, lang: &String, code: &String, message: &String) -> Result<()>;
}
