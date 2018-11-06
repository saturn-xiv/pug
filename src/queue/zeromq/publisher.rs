use zmq;

use super::super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn url(&self) -> String {
        format!("tcp://*:{}", self.port)
    }
}

pub struct Publisher {
    socket: zmq::Socket,
}

impl Publisher {
    pub fn new(url: &String) -> Result<Self> {
        let c = zmq::Context::new();
        let s = c.socket(zmq::PUB)?;
        s.bind(url)?;
        Ok(Self { socket: s })
    }
    pub fn send(&self, channel: &[u8], message: &[u8]) -> Result<()> {
        self.socket.send(channel, zmq::SNDMORE)?;
        self.socket.send(message, 0)?;
        Ok(())
    }
}
