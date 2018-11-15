pub mod settings;

use rocket_contrib::databases::diesel::SqliteConnection;

pub type DieselConnection = SqliteConnection;

#[database("sqlite")]
pub struct Connection(DieselConnection);
