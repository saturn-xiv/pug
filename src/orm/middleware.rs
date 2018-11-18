// https://github.com/gotham-rs/gotham/tree/master/middleware/under_development/diesel
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

use diesel::{r2d2::ConnectionManager, Connection};
use futures::{future, Future};
use gotham::{
    handler::HandlerFuture,
    middleware::{Middleware, NewMiddleware},
    state::{request_id, State},
};
use r2d2::{Error, Pool, PooledConnection};

#[derive(StateData)]
pub struct Diesel<T>
where
    T: Connection + 'static,
{
    pool: Pool<ConnectionManager<T>>,
}

impl<T> Diesel<T>
where
    T: Connection + 'static,
{
    pub(crate) fn new(pool: Pool<ConnectionManager<T>>) -> Self {
        Diesel { pool }
    }

    pub fn connection(&self) -> Result<PooledConnection<ConnectionManager<T>>, Error> {
        self.pool.get()
    }
}

pub struct DieselMiddleware<T>
where
    T: Connection + 'static,
{
    pool: AssertUnwindSafe<r2d2::Pool<ConnectionManager<T>>>,
}

pub struct DieselMiddlewareImpl<T>
where
    T: Connection + 'static,
{
    pool: r2d2::Pool<ConnectionManager<T>>,
}

impl<T> DieselMiddleware<T>
where
    T: Connection,
{
    pub fn new(pool: Pool<ConnectionManager<T>>) -> Self {
        DieselMiddleware {
            pool: AssertUnwindSafe(pool),
        }
    }
}

impl<T> NewMiddleware for DieselMiddleware<T>
where
    T: Connection + 'static,
{
    type Instance = DieselMiddlewareImpl<T>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => Ok(DieselMiddlewareImpl { pool }),
            Err(_) => {
                error!(
                    "PANIC: r2d2::Pool::clone caused a panic, unable to rescue with a HTTP error"
                );
                process::abort()
            }
        }
    }
}

impl<T> Clone for DieselMiddleware<T>
where
    T: Connection + 'static,
{
    fn clone(&self) -> Self {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => DieselMiddleware {
                pool: AssertUnwindSafe(pool),
            },
            Err(_) => {
                error!("PANIC: r2d2::Pool::clone caused a panic");
                process::abort()
            }
        }
    }
}

impl<T> Middleware for DieselMiddlewareImpl<T>
where
    T: Connection + 'static,
{
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        trace!("[{}] pre chain", request_id(&state));
        state.put(Diesel::<T>::new(self.pool));

        let f = chain(state).and_then(move |(state, response)| {
            {
                trace!("[{}] post chain", request_id(&state));
            }
            future::ok((state, response))
        });
        Box::new(f)
    }
}
