#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::{crypto::Encryptor, errors::Result};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

pub trait SettingDao {
    fn get<K: Serialize, V: DeserializeOwned, E: Encryptor>(&self, e: &E, key: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize, E: Encryptor>(
        &self,
        e: &E,
        k: &K,
        v: &V,
        f: bool,
    ) -> Result<()>;
}
