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

use super::super::orm::schema::New as Schema;

pub fn migration<'a>() -> Schema<'a> {
    Schema {
        version: "20181202020349647459641",
        name: "create-nut-site",
        up: UP,
        down: DOWN,
    }
}
