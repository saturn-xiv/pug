use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::{App, SubCommand};
use rocket_contrib::templates::tera::Tera;

use super::super::super::errors::Result;

pub const COMMAND_NAME: &'static str = "generate:systemd";
pub const COMMAND_ABOUT: &'static str = "Generate systemd service.conf";
pub const ARG_SERVICE_NAME: &'static str = "name";

#[derive(Serialize)]
struct Config {
    name: String,
    root: String,
    description: String,
}

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about(COMMAND_ABOUT)
        .arg(
            clap::Arg::with_name(ARG_SERVICE_NAME)
                .required(true)
                .short("n")
                .long("name")
                .value_name("SERVICE_NAME")
                .help("HTTP server name")
                .takes_value(true),
        )
}

pub fn run(name: String, description: String) -> Result<()> {
    let tpl = "systemd.conf";
    let mut tera = Tera::default();
    let cur = current_dir()?;
    tera.add_raw_template(tpl, include_str!("systemd.conf"))?;
    let buf = tera.render(
        tpl,
        &Config {
            name: name.clone(),
            description: description,
            root: format!("{}", cur.display()),
        },
    )?;

    let file = Path::new("tmp").join(format!("{}.service", name));
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    fd.write_all(buf.as_bytes())?;
    Ok(())
}
