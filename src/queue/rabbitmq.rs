use std::net::SocketAddr;

use failure::Error;
use futures::future::Future;
use futures::Stream;
use lapin::channel::{
    BasicConsumeOptions, BasicGetOptions, BasicProperties, BasicPublishOptions,
    ConfirmSelectOptions, QueueDeclareOptions,
};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use super::super::errors::Result;

pub struct RabbitMQ {
    addr: SocketAddr,
    options: ConnectionOptions,
}

impl RabbitMQ {
    pub fn new(addr: &str, username: String, password: String, vhost: String) -> Result<Self> {
        Ok(Self {
            addr: addr.parse()?,
            options: ConnectionOptions {
                username: username,
                password: password,
                vhost: vhost,
                frame_max: 65535,
                heartbeat: 20,
                ..Default::default()
            },
        })
    }
    pub fn publich(&self, queue: String, payload: Vec<u8>) -> Result<()> {
        let options = self.options.clone();
        let rst = Runtime::new()?.block_on_all(
            TcpStream::connect(&self.addr)
                .map_err(Error::from)
                .and_then(|stream| {
                    lapin::client::Client::connect(stream, options).map_err(Error::from)
                })
                .and_then(move |(client, heartbeat)| {
                    tokio::spawn(heartbeat.map_err(|e| error!("heartbeat error: {}", e)));

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
                                            BasicProperties::default(),
                                        )
                                        .map(|confirmation| {
                                            info!("publish got confirmation: {:?}", confirmation)
                                        })
                                })
                        })
                        .map_err(Error::from)
                })
                .map_err(Error::from),
        );

        match rst {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e).into()),
        }
    }

    pub fn consume(&self, consumer_name: String, queue_name: String) -> Result<()> {
        let options = self.options.clone();
        let rst = Runtime::new().unwrap().block_on_all(
            TcpStream::connect(&self.addr)
                .map_err(Error::from)
                .and_then(|stream| {
                    lapin::client::Client::connect(stream, options).map_err(Error::from)
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
                                                std::str::from_utf8(&message.delivery.data)
                                                    .unwrap()
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
                                        info!(
                                            "decoded message: {:?}",
                                            std::str::from_utf8(&message.data).unwrap()
                                        );
                                        c.basic_ack(message.delivery_tag, false)
                                    })
                                })
                        })
                        .map_err(Error::from)
                })
                .map_err(|err| eprintln!("An error occured: {}", err)),
        );
        match rst {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e).into()),
        }
    }
}
