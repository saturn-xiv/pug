pub mod log;
pub mod policy;
pub mod user;

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
        version: "20181202020324964361900",
        name: "create-nut-auth",
        up: UP,
        down: DOWN,
    }
}
