#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

use chrono::NaiveDateTime;
use mustache;
use serde::ser::Serialize;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use super::super::{
    errors::{Error, Result},
    orm::Connection,
};

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

pub trait Translate {
    fn tr<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Result<String>;
    fn e<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Error;
    fn t<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> String;
}

impl Translate for Connection {
    fn tr<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Result<String> {
        let msg = self.get(lang, code)?;
        if let Some(args) = args {
            let tpl = mustache::compile_str(&msg)?;
            return Ok(tpl.render_to_string(args)?);
        }
        return Ok(msg);
    }

    fn e<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Error {
        self.t(lang, code, args).into()
    }

    fn t<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> String {
        match self.tr(lang, code, args) {
            Ok(msg) => msg,
            Err(e) => {
                error!("{:?}", e);
                format!("{}.{}", lang, code)
            }
        }
    }
}
