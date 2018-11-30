use r2d2_redis::redis::Connection;

#[database("redis")]
pub struct Cache(Connection);
