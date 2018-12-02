#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate cookie;
extern crate encoding_rs;
extern crate eui48;
extern crate futures;
extern crate hyper;
extern crate jsonwebtoken;
extern crate language_tags;
extern crate log4rs;
extern crate mime;
extern crate mustache;
extern crate nix;
extern crate oping;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rust_sodium;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate toml;
extern crate url;
extern crate validator;
extern crate xml;
extern crate zmq;

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
