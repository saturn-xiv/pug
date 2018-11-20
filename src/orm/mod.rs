#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

pub mod schema;

use actix::prelude::*;
use diesel::r2d2::{
    ConnectionManager, Pool as DieselPool, PooledConnection as DieselPooledConnection,
};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

pub type Pool = DieselPool<ConnectionManager<Connection>>;
pub type PooledConnection = DieselPooledConnection<ConnectionManager<Connection>>;

pub struct DatabaseExecutor(Pool);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}
