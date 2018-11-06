use hyper::header::{Authorization, Bearer, Header, Host as HttpHost};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request,
};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host(pub Option<String>, pub Option<u16>);

impl<'a, 'r> FromRequest<'a, 'r> for Host {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let scheme = req.headers().get_one("X-Forwarded-Proto").unwrap_or("http");
        if let Some(host) = req.headers().get_one(HttpHost::header_name()) {
            if let Ok(url) = Url::parse(&format!("{}://{}", scheme, host)) {
                return Outcome::Success(Host(
                    match url.host_str() {
                        Some(h) => Some(h.to_string()),
                        None => None,
                    },
                    url.port(),
                ));
            }
        }

        Outcome::Failure((Status::BadRequest, ()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let items: Vec<Vec<u8>> = req
            .headers()
            .get(Authorization::<Bearer>::header_name())
            .map(|v| v.as_bytes().to_vec())
            .collect();
        if let Ok(auth) = Authorization::<Bearer>::parse_header(&items) {
            let Authorization::<Bearer>(bearer) = auth;
            return Outcome::Success(Token(Some(bearer.token)));
        }
        Outcome::Success(Token(None))
    }
}
