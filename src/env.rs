use std::fmt;
use std::path::{Path, PathBuf};

use base64;
use r2d2;
use url::Url;

#[cfg(any(feature = "redis"))]
use r2d2_redis;

use super::{
    crypto::{self, Encryptor},
    errors::Result,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Production,
    Staging,
    Development,
    Test,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Production => "production",
                Environment::Staging => "staging",
                Environment::Development => "development",
                Environment::Test => "test",
            }
        )
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    #[cfg(any(feature = "sodium"))]
    pub secrets: String,
    #[cfg(any(
        feature = "postgresql",
        feature = "mysql",
        feature = "sqlite"
    ))]
    pub database: String,
    #[cfg(feature = "redis")]
    pub redis: String,
    #[cfg(feature = "rabbitmq")]
    pub rabbitmq: String,
    pub http: Http,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: Environment::default(),
            http: Http::default(),
            #[cfg(feature = "sodium")]
            secrets: base64::encode(&crypto::sodium::Encryptor::random(32)),
            #[cfg(feature = "redis")]
            redis: "redis://localhost:5432/0".to_string(),
            #[cfg(feature = "sqlite")]
            database: "tmp/db".to_string(),
            #[cfg(feature = "mysql")]
            database: "mysql://root:@localhost:3306/pug".to_string(),
            #[cfg(feature = "postgresql")]
            database: "postgres://postgres:@localhost:5432/pug".to_string(),
            #[cfg(feature = "rabbitmq")]
            rabbitmq: "rabbitmq://".to_string(),
        }
    }
}

impl Config {
    #[cfg(any(feature = "sodium"))]
    pub fn secrets(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secrets)?;
        Ok(buf)
    }

    #[cfg(feature = "redis")]
    pub fn redis(&self) -> Result<r2d2::Pool<r2d2_redis::RedisConnectionManager>> {
        let manager = r2d2_redis::RedisConnectionManager::new(Url::parse(&self.redis)?)?;
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(pool)
    }

    #[cfg(any(
        feature = "postgresql",
        feature = "mysql",
        feature = "sqlite"
    ))]
    pub fn database(&self) -> Result<super::orm::Pool> {
        use diesel::r2d2::ConnectionManager;
        let manager = ConnectionManager::<super::orm::Connection>::new(&self.database[..]);
        let pool = super::orm::Pool::new(manager)?;
        Ok(pool)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub theme: String,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            theme: "bootstrap".to_string(),
        }
    }
}

impl Http {
    const THEMES: &'static str = "themes";

    pub fn global(&self) -> PathBuf {
        Path::new(Self::THEMES).join("global")
    }
    pub fn templates(&self) -> PathBuf {
        Path::new(Self::THEMES)
            .join(self.theme.clone())
            .join("views")
    }
    pub fn assets(&self) -> PathBuf {
        Path::new(Self::THEMES)
            .join(self.theme.clone())
            .join("assets")
    }
    pub fn third(&self) -> PathBuf {
        Path::new("node_modules").to_path_buf()
    }
}
