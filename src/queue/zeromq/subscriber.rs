use zmq;

use super::super::super::errors::Result;

pub struct Subscriber {
    socket: zmq::Socket,
}

impl Subscriber {
    pub fn new(url: &String, channels: Vec<String>) -> Result<Self> {
        let c = zmq::Context::new();
        let s = c.socket(zmq::SUB)?;
        s.connect(url)?;
        for it in channels {
            s.set_subscribe(it.as_bytes())?;
        }
        Ok(Self { socket: s })
    }
    pub fn start<F>(&self, f: F)
    where
        F: Fn(Vec<u8>, Vec<u8>) -> Result<()>,
    {
        let run = move || -> Result<()> {
            let envelope = self.socket.recv_bytes(0)?;
            let message = self.socket.recv_bytes(0)?;
            f(envelope, message)?;
            Ok(())
        };
        loop {
            if let Err(e) = run() {
                error!("fail on process: {:?}", e);
            }
        }
    }
}
