extern crate pug;

use std::thread;
use std::time::Duration;

#[test]
fn it_rabbitmq() {
    let queue = "echo";
    let mq = pug::queue::rabbitmq::RabbitMQ::new(
        "localhost:5672",
        "guest".to_string(),
        "guest".to_string(),
        "/pug".to_string(),
    )
    .unwrap();
    mq.publich(queue.to_string(), b"hello, pug!".to_vec())
        .unwrap();
    thread::sleep(Duration::from_secs(2));
    mq.consume("pug.test".to_string(), queue.to_string())
        .unwrap();
}
