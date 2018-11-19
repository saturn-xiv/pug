use std::sync::mpsc;
use std::thread;

use actix;
use actix_web::{server, App, HttpResponse};
use futures::Future;

pub fn start(port: u16) {
    let (tx, rx) = mpsc::channel();

    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
    thread::spawn(move || {
        let sys = actix::System::new("http-server");
        let addr = server::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
            .bind(addr)
            .expect("Can not bind")
            .shutdown_timeout(60)
            .start();
        let _ = tx.send(addr);
        let _ = sys.run();
    });

    let app = rx.recv().unwrap();
    let _ = app.send(server::StopServer { graceful: true }).wait();
}
