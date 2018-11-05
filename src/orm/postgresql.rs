use rocket_contrib::databases::diesel;

#[database("postgresql")]
pub struct Connection(diesel::PgConnection);
