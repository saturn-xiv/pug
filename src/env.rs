use std::path::{Path, PathBuf};

use base64;
use r2d2;
use rocket::config::{Config as RocketConfig, Environment, Limits, LoggingLevel};
use url::Url;

#[cfg(any(feature = "redis"))]
use r2d2_redis;

use super::{
    crypto::{self, Encryptor},
    errors::Result,
};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: String,
    #[cfg(any(feature = "sodium"))]
    pub secrets: String,
    #[cfg(any(feature = "postgresql", feature = "mysql", feature = "sqlite"))]
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
            env: format!("{}", Environment::Development),
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
    pub fn env(&self) -> Environment {
        match self.env.parse() {
            Ok(v) => v,
            Err(_) => Environment::Development,
        }
    }

    pub fn rocket(&self) -> Result<RocketConfig> {
        let env = self.env();
        let it = RocketConfig::build(env)
            .address("0.0.0.0")
            .workers(self.http.workers)
            .port(self.http.port)
            .secret_key(&self.secrets[..])
            .keep_alive(match self.http.keep_alive {
                Some(v) => v,
                None => 0,
            })
            .limits(
                Limits::new()
                    .limit("forms", self.http.limits * (1 << 10 << 10))
                    .limit("json", self.http.limits * (1 << 10 << 10)),
            )
            .extra(
                "template_dir",
                match self.http.templates().to_str() {
                    Some(v) => v,
                    None => "templates",
                },
            )
            .extra("database", &self.database[..])
            .log_level(match env {
                Environment::Production => LoggingLevel::Normal,
                _ => LoggingLevel::Debug,
            })
            .workers(12)
            .finalize()?;
        Ok(it)
    }

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

    #[cfg(any(feature = "postgresql", feature = "mysql", feature = "sqlite"))]
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
    pub workers: u16,
    pub limits: u64,
    pub keep_alive: Option<u32>,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 64,
            theme: "bootstrap".to_string(),
            limits: 20,
            keep_alive: Some(120),
        }
    }
}

impl Http {
    const THEMES: &'static str = "themes";

    pub fn address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }

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
