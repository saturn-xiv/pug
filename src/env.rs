use base64;
use rocket::{
    config::{Config as RocketConfig, Environment, LoggingLevel, Table, Value},
    custom as rocket_custom, Rocket,
};

use super::{
    crypto::{self, Encryptor},
    errors::Result,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: String,
    pub secrets: String,
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

    fn database_url(&self, v: String) -> Value {
        let mut it = Table::new();
        it.insert("url".to_string(), Value::String(v));
        Value::Table(it)
    }

    pub fn rocket(&self) -> Result<Rocket> {
        let env = self.env();

        let mut databases = Table::new();
        #[cfg(feature = "redis")]
        {
            databases.insert("redis".to_string(), self.database_url(self.redis.clone()));
        }
        #[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgresql"))]
        {
            databases.insert(
                "database".to_string(),
                self.database_url(self.database.clone()),
            );
        }

        let cfg = RocketConfig::build(env)
            .address("0.0.0.0")
            .port(self.http.port)
            .workers(self.http.workers)
            .secret_key(self.secrets.clone())
            .keep_alive(self.http.keep_alive)
            .log_level(if env.is_prod() {
                LoggingLevel::Critical
            } else {
                LoggingLevel::Debug
            })
            .extra("template_dir", format!("themes/{}/views", self.http.theme))
            .extra("assets_dir", format!("themes/{}/assets", self.http.theme))
            .extra("databases", databases)
            .limits(
                rocket::config::Limits::new()
                    .limit("forms", self.http.forms_limit * 1024)
                    .limit("json", self.http.json_limit * 1024),
            );

        let mut app = rocket_custom(cfg.finalize()?);
        #[cfg(feature = "redis")]
        {
            app = app.attach(super::redis::Connection::fairing());
        }
        #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgresql"))]
        {
            app = app.attach(super::orm::Connection::fairing());
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
