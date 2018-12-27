pub mod rabbitmq;
pub mod redis;
pub mod zeromq;

use std::result::Result;

pub trait Queue {
    type Error;
    fn publish(&self, name: String, id: String, payload: Vec<u8>) -> Result<(), Self::Error>;
    fn consume(
        &self,
        consumer: String,
        queue: String,
        handler: Box<Handler<Error = Self::Error>>,
    ) -> Result<(), Self::Error>;
}

pub trait Handler: Sync + Send {
    type Error;
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<(), Self::Error>;
}
