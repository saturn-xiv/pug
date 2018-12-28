use serde_json;

use super::super::super::{errors::Result, queue::Handler};

pub const NAME: &'static str = "send-email";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    host: String,
    port: u16,
    user: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub struct Consumer {}

impl Handler for Consumer {
    fn handle(&self, _id: String, payload: Vec<u8>) -> Result<()> {
        self.run(&serde_json::from_slice(&payload)?)
    }
}

impl Consumer {
    #[cfg(debug_assertions)]
    fn run(&self, task: &Task) -> Result<()> {
        info!("send email: {:?}", task);
        Ok(())
    }
    #[cfg(not(debug_assertions))]
    fn run(&self, task: &Task) -> Result<()> {
        info!("send email: {} {}", task.to, task.subject);
        Ok(())
    }
}
