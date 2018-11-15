use rocket_contrib::databases::diesel::PgConnection;

pub type DieselConnection = PgConnection;

#[database("database")]
pub struct Connection(DieselConnection);
