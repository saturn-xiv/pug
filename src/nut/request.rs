use hyper::header::Header;
use rocket::{
    http::{
        hyper::header::{Authorization, Bearer, Host as HyperHost},
        Status,
    },
    request::{self, FromRequest},
    Outcome, Request,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub hostname: String,
    pub port: Option<u16>,
}

impl From<HyperHost> for Host {
    fn from(v: HyperHost) -> Self {
        Self {
            hostname: v.hostname,
            port: v.port,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Host {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if let Some(host) = req.headers().get_one(HyperHost::header_name()) {
            if let Ok(host) = host.parse::<HyperHost>() {
                return Outcome::Success(host.into());
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
        if let Some(auth) = req
            .headers()
            .get_one(Authorization::<Bearer>::header_name())
        {
            if let Ok(auth) = auth.parse::<Bearer>() {
                return Outcome::Success(Token(Some(auth.token)));
            }
        }
        Outcome::Success(Token(None))
    }
}
