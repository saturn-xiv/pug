use rocket_contrib::databases::diesel;

#[database("sqlite")]
pub struct Connection(diesel::SqliteConnection);
