use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::{App, SubCommand};
use rocket_contrib::templates::tera::Tera;

use super::super::super::errors::Result;

pub const COMMAND_NAME: &'static str = "generate:nginx";
pub const ARG_HTTPS: &'static str = "https";
pub const ARG_SERVER_NAME: &'static str = "server_name";

#[derive(Serialize)]
struct Config {
    name: String,
    port: u16,
    ssl: bool,
    root: String,
}

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Generate nginx.conf")
        .arg(
            clap::Arg::with_name(ARG_HTTPS)
                .short("s")
                .long("ssl")
                .value_name("HTTPS")
                .help("Enable https?")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name(ARG_SERVER_NAME)
                .required(true)
                .short("n")
                .long("server-name")
                .value_name("SERVER_NAME")
                .help("HTTP server name")
                .takes_value(true),
        )
}

pub fn run(name: String, port: u16, ssl: bool) -> Result<()> {
    let tpl = "nginx.conf";
    let mut tera = Tera::default();
    let cur = current_dir()?;
    tera.add_raw_template(tpl, include_str!("nginx.conf"))?;
    let buf = tera.render(
        tpl,
        &Config {
            name: name,
            port: port,
            ssl: ssl,
            root: format!("{}", cur.display()),
        },
    )?;

    let file = Path::new("tmp").join(tpl);
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    fd.write_all(buf.as_bytes())?;
    Ok(())
}
