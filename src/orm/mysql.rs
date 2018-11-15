use rocket_contrib::databases::diesel::MysqlConnection;

pub type DieselConnection = MysqlConnection;

#[database("database")]
pub struct Connection(DieselConnection);
