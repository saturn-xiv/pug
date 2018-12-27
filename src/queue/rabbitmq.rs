use std::net::SocketAddr;

use failure::Error as FailureError;
use futures::{future::Future, Stream};
use lapin::channel::{
    BasicConsumeOptions, BasicGetOptions, BasicProperties, BasicPublishOptions,
    ConfirmSelectOptions, QueueDeclareOptions,
};
use lapin::{client::ConnectionOptions, types::FieldTable};
use tokio::{net::TcpStream, runtime::Runtime};

use super::super::errors::{Error, Result};
use super::{Handler, Queue};

pub struct RabbitMQ {
    addr: SocketAddr,
    options: ConnectionOptions,
}

impl RabbitMQ {
    pub fn new(addr: String, username: String, password: String, vhost: String) -> Result<Self> {
        Ok(Self {
            addr: addr.parse()?,
            options: ConnectionOptions {
                username: username,
                password: password,
                vhost: vhost,
                frame_max: std::u16::MAX as u32,
                ..Default::default()
            },
        })
    }
}
impl Queue for RabbitMQ {
    type Error = Error;
    fn publish(&self, queue: String, mid: String, payload: Vec<u8>) -> Result<()> {
        let options = self.options.clone();

        let rt = TcpStream::connect(&self.addr)
            .map_err(FailureError::from)
            .and_then(|stream| {
                lapin::client::Client::connect(stream, options).map_err(FailureError::from)
            })
            .and_then(move |(client, _)| {
                // tokio::spawn(heartbeat.map_err(|e| error!("heartbeat error: {}", e)));

                client
                    .create_confirm_channel(ConfirmSelectOptions::default())
                    .and_then(move |channel| {
                        let id = channel.id;
                        info!("created channel with id: {}", id);

                        channel
                            .queue_declare(
                                &queue.clone(),
                                QueueDeclareOptions::default(),
                                FieldTable::new(),
                            )
                            .and_then(move |_| {
                                info!("channel {} declared queue {}", id, queue);
                                channel
                                    .basic_publish(
                                        "",
                                        &queue,
                                        payload,
                                        BasicPublishOptions::default(),
                                        BasicProperties::default().with_message_id(mid),
                                    )
                                    .map(|confirmation| {
                                        info!("publish got confirmation: {:?}", confirmation)
                                    })
                                    .and_then(move |_| {
                                        info!("close channel");
                                        channel.close_ok()
                                    })
                            })
                    })
                    .map_err(FailureError::from)
            });

        if let Err(e) = Runtime::new()?.block_on_all(rt) {
            return Err(format!("failed on publish message {}", e).into());
        }
        Ok(())
    }

    fn consume(
        &self,
        consumer_name: String,
        queue_name: String,
        handler: Box<Handler<Error = Self::Error>>,
    ) -> Result<()> {
        let options = self.options.clone();

        let rt = TcpStream::connect(&self.addr)
            .map_err(FailureError::from)
            .and_then(|stream| {
                lapin::client::Client::connect(stream, options).map_err(FailureError::from)
            })
            .and_then(move |(client, heartbeat)| {
                tokio::spawn(heartbeat.map_err(|e| error!("heartbeat error: {}", e)));

                client
                    .create_channel()
                    .and_then(move |channel| {
                        let id = channel.id;
                        info!("created channel with id: {}", id);

                        let c = channel.clone();
                        channel
                            .queue_declare(
                                &queue_name.clone(),
                                QueueDeclareOptions::default(),
                                FieldTable::new(),
                            )
                            .and_then(move |queue| {
                                info!("channel {} declared queue {:?}", id, queue);

                                let ch = channel.clone();
                                channel
                                    .basic_get(&queue_name, BasicGetOptions::default())
                                    .and_then(move |message| {
                                        info!("got message: {:?}", message);
                                        info!(
                                            "decoded message: {:?}",
                                            std::str::from_utf8(&message.delivery.data).unwrap()
                                        );
                                        channel.basic_ack(message.delivery.delivery_tag, false)
                                    })
                                    .and_then(move |_| {
                                        ch.basic_consume(
                                            &queue,
                                            &consumer_name,
                                            BasicConsumeOptions::default(),
                                            FieldTable::new(),
                                        )
                                    })
                            })
                            .and_then(|stream| {
                                info!("got consumer stream");
                                stream.for_each(move |message| {
                                    debug!("got message: {:?}", message);
                                    match message.properties.message_id() {
                                        Some(id) => {
                                            if let Err(e) =
                                                handler.handle(id.to_string(), message.data)
                                            {
                                                error!("failed on process message {:?}", e);
                                            }
                                        }
                                        None => error!("bad message"),
                                    };
                                    c.basic_ack(message.delivery_tag, false)
                                })
                            })
                    })
                    .map_err(FailureError::from)
            });

        match Runtime::new()?.block_on_all(rt) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("failed on consume message: {:?}", e).into()),
        }
    }
}
