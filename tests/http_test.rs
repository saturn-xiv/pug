extern crate pug;

#[test]
fn it_http_server() {
    pug::http::router::listen(8080);
}
