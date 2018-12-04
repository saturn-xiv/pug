pub mod db;

use mustache;
use rocket::{
    request::{self, FromRequest},
    Outcome, Request,
};
use serde::ser::Serialize;

use super::{
    errors::{Error, Result},
    orm::{Database, PooledConnection},
};

pub trait Provider {
    fn get(&self, lang: &String, code: &String) -> Option<String>;
    fn exist(&self, lang: &String) -> bool;
}

pub struct I18n {
    providers: Vec<Box<Provider>>,
}

impl I18n {
    pub fn new(db: PooledConnection) -> Self {
        Self {
            providers: vec![Box::new(db)],
        }
    }
    pub fn exist(&self, lang: &String) -> bool {
        for it in self.providers.iter() {
            if it.exist(lang) {
                return true;
            }
        }
        false
    }
    pub fn tr<S: Serialize>(
        &self,
        lang: &String,
        code: &String,
        args: &Option<S>,
    ) -> Result<Option<String>> {
        for it in self.providers.iter() {
            if let Some(msg) = it.get(lang, code) {
                let tpl = mustache::compile_str(&msg)?;
                return Ok(Some(tpl.render_to_string(args)?));
            }
        }
        Ok(None)
    }

    pub fn e<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Error {
        self.t(lang, code, args).into()
    }

    pub fn t<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> String {
        if let Some(msg) = match self.tr(lang, code, args) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                None
            }
        } {
            return msg;
        }
        format!("{}.{}", lang, code)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for I18n {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Database(db) = request.guard::<Database>()?;
        Outcome::Success(I18n::new(db))
    }
}
