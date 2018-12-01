pub mod admin;
pub mod auth;
pub mod controllers;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push(("/api/users", routes![controllers::users::sign_out]));
        items
    };
}
