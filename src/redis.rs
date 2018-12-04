use r2d2_redis;

pub type Connection = r2d2_redis::redis::Connection;
pub type PooledConnection = r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

#[database("redis")]
pub struct Redis(pub Connection);
