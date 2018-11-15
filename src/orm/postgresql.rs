use rocket_contrib::databases::diesel::PgConnection;

pub type DieselConnection = PgConnection;
#[database("postgresql")]
pub struct Connection(DieselConnection);
