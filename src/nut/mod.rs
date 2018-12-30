pub mod auth;
pub mod catchers;
pub mod controllers;
pub mod request;
pub mod seo;
pub mod site;
pub mod tasks;

use std::fmt;
use std::str::FromStr;

use rocket::Route;

use super::errors::{Error, Result};

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api/users",
            routes![
                controllers::users::sign_in,
                controllers::users::sign_up,
                controllers::users::confirm,
                controllers::users::confirm_token,
                controllers::users::forgot_password,
                controllers::users::unlock,
                controllers::users::unlock_token,
                controllers::users::reset_password,
                controllers::users::logs,
                controllers::users::get_profile,
                controllers::users::post_profile,
                controllers::users::change_password,
                controllers::users::sign_out
            ],
        ));
        items.push((
            "/",
            routes![
                seo::robots::txt,
                seo::sitemap::xml_gz,
                seo::rss::atom,
                controllers::home
            ],
        ));
        items
    };
}

pub enum MediaType {
    TEXT,
    HTML,
    MARKDOWN,
}

impl fmt::Display for MediaType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaType::TEXT => write!(fmt, "text"),
            MediaType::HTML => write!(fmt, "html"),
            MediaType::MARKDOWN => write!(fmt, "markdown"),
        }
    }
}

impl FromStr for MediaType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(MediaType::TEXT),
            "markdown" => Ok(MediaType::MARKDOWN),
            "html" => Ok(MediaType::HTML),
            t => Err(format!("unknown media type {}", t).into()),
        }
    }
}
