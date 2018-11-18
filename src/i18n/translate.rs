use mustache;
use serde::ser::Serialize;

use super::super::{
    errors::{Error, Result},
    orm::Connection,
};
use super::locales::LocaleDao;

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
