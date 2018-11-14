pub mod generate;

use env_logger;
use log4rs;

#[cfg(feature = "sodium")]
use rust_sodium;

use clap::App;

pub fn new<'a, 'b>(
    name: &'a str,
    version: &'a str,
    author: Option<&'a str>,
    about: Option<&'a str>,
    banner: Option<&'a str>,
    homepage: Option<&'a str>,
) -> clap::App<'a, 'b> {
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        env_logger::init();
        error!("failed to parse log4rs.yml, {:?}", e);
    }
    #[cfg(feature = "sodium")]
    {
        if let Err(_) = rust_sodium::init() {
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
