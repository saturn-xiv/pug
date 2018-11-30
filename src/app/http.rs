pub fn start(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
}
