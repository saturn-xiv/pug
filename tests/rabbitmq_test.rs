extern crate env_logger;
extern crate pug;
extern crate serde;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::time::Duration;

use pug::{
    errors::Result,
    queue::{rabbitmq::RabbitMQ, Handler, Queue},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    message: String,
}

struct Echo;

impl Handler for Echo {
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<()> {
        let it: Task = serde_json::from_slice(&payload)?;
        println!("receive message {} {:?}", id, it);
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
            Task {
                message: format!("hello, {} pug!", i),
            },
        )
        .unwrap();
    }
    println!("wait for consume messages");
    thread::sleep(Duration::from_secs(2));
    mq.consume("pug.test".to_string(), queue.to_string(), Box::new(Echo {}))
        .unwrap();
}
