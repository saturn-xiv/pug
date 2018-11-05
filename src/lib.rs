#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
pub extern crate rocket;
#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate error_chain;

pub extern crate chrono;
pub extern crate chrono_tz;
pub extern crate jsonwebtoken;
pub extern crate serde;

pub mod app;
pub mod cache;
pub mod crypt;
pub mod env;
pub mod errors;
pub mod http;
pub mod i18n;
pub mod jwt;
pub mod orm;
