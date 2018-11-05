pub mod generate;

use log4rs;
#[cfg(feature = "sodium")]
use sodiumoxide;

use clap::{App, Arg, SubCommand};

pub fn new<'a, 'b>(
    name: &'a str,
    version: &'a str,
    author: Option<&'a str>,
    about: Option<&'a str>,
    banner: Option<&'a str>,
    homepage: Option<&'a str>,
) -> clap::App<'a, 'b> {
    // log4rs::init_file("log4rs.yml", Default::default())?;
    #[cfg(feature = "sodium")]
    {
        if let Err(_) = sodiumoxide::init() {
            error!("sodium init");
        }
    }
    let mut app = App::new(name).version(version);
    if let Some(v) = author {
        app = app.author(v);
    }
    if let Some(v) = about {
        app = app.about(v);
    }
    if let Some(v) = banner {
        app = app.before_help(v);
    }
    if let Some(v) = homepage {
        app = app.after_help(v);
    }
    app
}
