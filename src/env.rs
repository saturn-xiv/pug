use std::path::Path;
use std::{fs, io::Read};

use base64;
use rocket::{
    config::{Config as RocketConfig, Environment},
    custom as rocket_custom, Rocket,
};
use toml;

use super::{
    crypto::{self, Encryptor},
    errors::Result,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: String,
    pub secrets: String,
    #[cfg(feature = "redis")]
    pub redis: String,
    #[cfg(feature = "sqlite")]
    pub sqlite: String,
    #[cfg(feature = "mysql")]
    pub mysql: String,
    #[cfg(feature = "postgresql")]
    pub postgresql: String,
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
            redis: "tcp://localhost:5432/0".to_string(),
            #[cfg(feature = "sqlite")]
            sqlite: "tmp/db".to_string(),
            #[cfg(feature = "mysql")]
            mysql: "mysql://".to_string(),
            #[cfg(feature = "postgresql")]
            postgresql: "postgres://".to_string(),
            #[cfg(feature = "rabbitmq")]
            rabbitmq: "rabbitmq://".to_string(),
        }
    }
}

impl Config {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        info!("load config from file {}", file.as_ref().display());
        let mut file = fs::File::open(file)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let cfg = toml::from_slice(&buf)?;
        Ok(cfg)
    }

    pub fn secrets(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secrets)?;
        Ok(buf)
    }

    pub fn env(&self) -> Environment {
        match self.env.parse() {
            Ok(v) => v,
            Err(_) => {
                warn!(
                    "bad environment {:?}, using {:?} as default",
                    self.env,
                    Environment::Development
                );
                Environment::Development
            }
        }
    }

    pub fn rocket(&self) -> Result<Rocket> {
        let env = self.env();
        let mut cfg = RocketConfig::build(env)
            .address("0.0.0.0")
            .port(self.http.port)
            .workers(self.http.workers)
            .secret_key(self.secrets.clone())
            .keep_alive(self.http.keep_alive)
            .extra("template_dir", format!("themes/{}/views", self.http.theme))
            .extra("assets_dir", format!("themes/{}/assets", self.http.theme))
            .limits(
                rocket::config::Limits::new()
                    .limit("forms", self.http.forms_limit * 1024)
                    .limit("json", self.http.json_limit * 1024),
            );
        #[cfg(feature = "redis")]
        {
            cfg = cfg.extra("databases.redis", self.redis.clone());
        }
        #[cfg(feature = "sqlite")]
        {
            cfg = cfg.extra("databases.sqlite", self.sqlite.clone());
        }
        #[cfg(feature = "mysql")]
        {
            cfg = cfg.extra("databases.mysql", self.mysql.clone());
        }
        #[cfg(feature = "postgresql")]
        {
            cfg = cfg.extra("databases.postgresql", self.postgresql.clone());
        }

        let mut app = rocket_custom(cfg.finalize()?);
        #[cfg(feature = "redis")]
        {
            app = app.attach(super::redis::Connection::fairing());
        }
        #[cfg(feature = "sqlite")]
        {
            app = app.attach(super::orm::sqlite::Connection::fairing());
        }
        #[cfg(feature = "mysql")]
        {
            app = app.attach(super::orm::mysql::Connection::fairing());
        }
        #[cfg(feature = "postgresql")]
        {
            app = app.attach(super::orm::postgresql::Connection::fairing());
        }
        Ok(app)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub workers: u16,
    pub theme: String,
    pub forms_limit: u64,
    pub json_limit: u64,
    pub keep_alive: u32,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 8,
            keep_alive: 8,
            forms_limit: 1 << 8,
            json_limit: 1 << 10,
            theme: "bootstrap".to_string(),
        }
    }
}
