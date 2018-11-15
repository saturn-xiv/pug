#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
pub extern crate rocket;
#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate error_chain;
#[macro_use]
pub extern crate rocket_contrib;
#[macro_use]
pub extern crate serde_json;
#[macro_use]
pub extern crate log;
#[macro_use]
pub extern crate lazy_static;
#[macro_use]
pub extern crate validator_derive;
#[macro_use]
pub extern crate hyper;
#[macro_use]
pub extern crate diesel;

pub use self::error_chain::{
    error_chain, error_chain_processing, impl_error_chain_kind, impl_error_chain_processed,
    impl_extract_backtrace,
};
pub use self::hyper::{__hyper__deref, header};
pub use self::lazy_static::lazy_static;
pub use self::log::{debug, error, info, warn};
pub use self::serde_derive::{Deserialize, Serialize};
pub use self::validator_derive::Validate;

#[cfg(feature = "sodium")]
pub extern crate rust_sodium;
#[cfg(feature = "zeromq")]
pub extern crate zmq;

pub extern crate base64;
pub extern crate chrono;
pub extern crate chrono_tz;
pub extern crate clap;
pub extern crate encoding_rs;
pub extern crate env_logger;
pub extern crate jsonwebtoken;
pub extern crate libc;
pub extern crate log4rs;
pub extern crate oping;
pub extern crate rand;
pub extern crate regex;
pub extern crate reqwest;
pub extern crate serde;
pub extern crate serde_xml_rs;
pub extern crate sha2;
pub extern crate toml;
pub extern crate url;
pub extern crate validator;
pub extern crate xml;

pub mod app;
pub mod cache;
pub mod crypto;
pub mod env;
pub mod errors;
pub mod http;
pub mod i18n;
pub mod jwt;
pub mod nut;
pub mod orm;
pub mod parser;
pub mod queue;
pub mod sys;

#[cfg(feature = "redis")]
pub mod redis;
