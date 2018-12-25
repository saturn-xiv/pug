pub mod rabbitmq;
pub mod redis;
pub mod zeromq;

use std::collections::HashMap;

use super::errors::Result;

pub trait Queue {
    fn publish(&self, name: String, payload: Vec<u8>) -> Result<()>;
    fn consumer(&self, name: String, handlers: HashMap<String, Box<Handler>>) -> Result<()>;
}

pub trait Handler {
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<()>;
}
