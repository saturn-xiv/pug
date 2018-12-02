#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Log {
    #[cfg(feature = "sqlite")]
    pub id: i32,
    #[cfg(any(feature = "postgresql", feature = "mysql"))]
    pub id: i64,
    pub ip: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}
