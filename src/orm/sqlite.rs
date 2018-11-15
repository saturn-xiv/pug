use rocket_contrib::databases::diesel::SqliteConnection;

pub type DieselConnection = SqliteConnection;

#[database("database")]
pub struct Connection(DieselConnection);
