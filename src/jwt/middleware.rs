use std::{io, marker::PhantomData, panic::RefUnwindSafe};

use futures::{future, Future};
use gotham::{
    handler::HandlerFuture,
    helpers::http::response::create_empty_response,
    middleware::{Middleware, NewMiddleware},
    state::{request_id, FromState, State},
};
use hyper::{
    header::{HeaderMap, AUTHORIZATION},
    StatusCode,
};
use jsonwebtoken::{decode, TokenData, Validation};
use serde::de::Deserialize;

#[derive(StateData)]
pub struct AuthorizationToken<T>
where
    T: Send + 'static,
{
    pub token: TokenData<T>,
}

impl<T> AuthorizationToken<T>
where
    T: Send + 'static,
{
    pub fn new(token: TokenData<T>) -> Self {
        AuthorizationToken { token }
    }
}

pub struct Jwt<T> {
    secret: Vec<u8>,
    validation: Validation,
    claims: PhantomData<T>,
}

impl<T> Jwt<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    pub fn new(secret: Vec<u8>) -> Self {
        let validation = Validation::default();

        Jwt {
            secret,
            validation,
            claims: PhantomData,
        }
    }

    pub fn validation(self, validation: Validation) -> Self {
        Jwt { validation, ..self }
    }
}

impl<T> Middleware for Jwt<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        trace!("[{}] pre-chain authentication", request_id(&state));

        let token: Option<String> = {
            let header = HeaderMap::borrow_from(&state).get(AUTHORIZATION);

            match header {
                Some(h) => match h.to_str() {
                    Ok(hx) => {
                        let parts: Vec<&str> = hx.rsplit(": ").collect();
                        Some(parts[0].to_owned())
                    }
                    Err(_) => None,
                },
                None => None,
            }
        };

        if token.is_none() {
            let res = create_empty_response(&state, StatusCode::BAD_REQUEST);
            return Box::new(future::ok((state, res)));
        }

        match decode::<T>(&token.unwrap(), self.secret.as_ref(), &self.validation) {
            Ok(token) => {
                state.put(AuthorizationToken::<T>::new(token));

                let res = chain(state).and_then(|(state, res)| {
                    trace!("[{}] post-chain jwt middleware", request_id(&state));
                    future::ok((state, res))
                });

                Box::new(res)
            }
            Err(e) => {
                trace!("[{}] error jwt middleware", e);
                let res = create_empty_response(&state, StatusCode::UNAUTHORIZED);
                Box::new(future::ok((state, res)))
            }
        }
    }
}

impl<T> NewMiddleware for Jwt<T>
where
    T: for<'de> Deserialize<'de> + RefUnwindSafe + Send + Sync + 'static,
{
    type Instance = Jwt<T>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(Jwt {
            secret: self.secret.clone(),
            validation: self.validation.clone(),
            claims: PhantomData,
        })
    }
}
