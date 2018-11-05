use rocket_contrib::databases::redis;

#[database("redis")]
pub struct Connection(redis::Connection);
