use rocket_contrib::databases::diesel;

#[database("mysql")]
pub struct Connection(diesel::MysqlConnection);
