use std::str::FromStr;

use cookie::Cookie;
use futures::{future, Future};
use gotham::{
    handler::HandlerFuture,
    middleware::Middleware,
    state::{FromState, State},
};
use hyper::{
    header::{ACCEPT_LANGUAGE, COOKIE},
    HeaderMap,
};
use language_tags::LanguageTag;

use super::super::{errors::Result, orm::Middleware as Database};
use super::locales::LocaleDao;

#[derive(StateData)]
pub struct LocaleMiddlewareData {
    pub name: String,
}

#[derive(Clone, NewMiddleware)]
pub struct LocaleMiddleware;

impl LocaleMiddleware {
    const EN_US: &'static str = "en-US";
    fn parse(headers: &HeaderMap) -> Result<Option<String>> {
        // 1. Check URL arguments.
        // 2. Get language information from cookies.
        if let Some(ck) = headers
            .get_all(COOKIE)
            .iter()
            .flat_map(|hv| hv.to_str())
            .flat_map(|cv| Cookie::parse(cv.to_owned()))
            .find(|cookie| cookie.name() == "locale")
            .map(|adj_cookie| adj_cookie.value().to_owned())
        {
            return Ok(Some(ck));
        }
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(al) = headers.get(ACCEPT_LANGUAGE) {
            if let Some(lng) = LanguageTag::from_str(al.to_str()?)?.language {
                return Ok(Some(lng));
            }
        }
        Ok(None)
    }

    fn validate(headers: &HeaderMap, db: &Database) -> Result<Option<String>> {
        let lang = Self::parse(headers)?;
        if let Some(lang) = lang {
            if db.connection()?.languages()?.contains(&lang) {
                return Ok(Some(lang));
            }
        }
        Ok(None)
    }
    fn detect(headers: &HeaderMap, db: &Database) -> String {
        match Self::validate(headers, db) {
            Ok(v) => match v {
                Some(v) => v,
                None => Self::EN_US.to_string(),
            },
            Err(e) => {
                error!("bad in detect locale: {:?}", e);
                Self::EN_US.to_string()
            }
        }
    }
}

impl Middleware for LocaleMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let lang = {
            let headers = HeaderMap::borrow_from(&state);
            let db = Database::borrow_from(&state);
            Self::detect(&headers, &db)
        };

        state.put(LocaleMiddlewareData { name: lang });

        let result = chain(state);

        let f = result.and_then(move |(state, response)| {
            {};
            future::ok((state, response))
        });

        Box::new(f)
    }
}
