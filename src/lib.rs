#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
pub extern crate rocket;
#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate error_chain;
#[macro_use]
pub extern crate serde_json;
#[macro_use]
pub extern crate log;
#[macro_use]
pub extern crate lazy_static;
#[macro_use]
pub extern crate validator_derive;
#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate hyper;

#[macro_use]
extern crate rocket_contrib;

pub extern crate base64;
pub extern crate chrono;
pub extern crate chrono_tz;
pub extern crate clap;
pub extern crate cookie;
pub extern crate encoding_rs;
pub extern crate env_logger;
pub extern crate futures;
pub extern crate jsonwebtoken;
pub extern crate language_tags;
pub extern crate libc;
pub extern crate log4rs;
pub extern crate mime;
pub extern crate mustache;
pub extern crate oping;
pub extern crate r2d2;
pub extern crate r2d2_redis;
pub extern crate rand;
pub extern crate regex;
pub extern crate reqwest;
pub extern crate rust_sodium;
pub extern crate serde;
pub extern crate serde_xml_rs;
pub extern crate tokio;
pub extern crate tokio_codec;
pub extern crate tokio_io;
pub extern crate toml;
pub extern crate url;
pub extern crate validator;
pub extern crate xml;
pub extern crate zmq;

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
pub mod redis;
pub mod rfc;
pub mod settings;
pub mod sys;

pub use self::diesel::{
    date_time_expr, numeric_expr, operator_allowed, QueryId, __diesel_column,
    __diesel_generate_ops_impls_if_date_time, __diesel_generate_ops_impls_if_numeric,
    __diesel_table_impl, __diesel_table_query_source_impl, __diesel_use_everything, static_cond,
    table, table_body,
};
pub use self::error_chain::{
    error_chain, error_chain_processing, impl_error_chain_kind, impl_error_chain_processed,
    impl_extract_backtrace,
};
pub use self::hyper::header;
pub use self::lazy_static::lazy_static;
pub use self::log::{debug, error, info, warn};
pub use self::serde_derive::{Deserialize, Serialize};
pub use self::validator_derive::Validate;
