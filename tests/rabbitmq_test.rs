extern crate env_logger;
extern crate pug;
extern crate uuid;

use std::thread;
use std::time::Duration;

use pug::{
    errors::{Error, Result},
    queue::{rabbitmq::RabbitMQ, Handler, Queue},
};
use uuid::Uuid;

struct Echo;

impl Handler for Echo {
    type Error = Error;
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<()> {
        println!("receive message {} {}", id, String::from_utf8(payload)?);
        Ok(())
    }
}

#[test]
fn it_rabbitmq() {
    env_logger::init();

    let queue = "echo";
    let mq = RabbitMQ::new(
        "127.0.0.1:5672".to_string(),
        "guest".to_string(),
        "guest".to_string(),
        "/pug".to_string(),
    )
    .unwrap();
    for i in 1..10 {
        println!("send message {}@{}", i, queue);
        mq.publish(
            queue.to_string(),
            Uuid::new_v4().to_string(),
            format!("hello, {} pug!", i).as_bytes().to_vec(),
        )
        .unwrap();
    }
    println!("wait for consume messages");
    thread::sleep(Duration::from_secs(2));
    mq.consume("pug.test".to_string(), queue.to_string(), Box::new(Echo {}))
        .unwrap();
}
