use rocket_contrib::databases::diesel::MysqlConnection;

pub type DieselConnection = MysqlConnection;

#[database("mysql")]
pub struct Connection(DieselConnection);
