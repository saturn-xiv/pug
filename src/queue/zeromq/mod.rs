pub mod publisher;
pub mod subscriber;

pub fn by_socket<S: AsRef<str>>(s: S) -> String {
    format!("ipc:////tmp/.{}.sock", s.as_ref())
}

pub fn by_port(h: Option<String>, p: u16) -> String {
    format!(
        "tcp://{}:{}",
        match h {
            Some(v) => v,
            None => "*".to_string(),
        },
        p
    )
}
