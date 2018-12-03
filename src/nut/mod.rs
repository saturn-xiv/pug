pub mod auth;
pub mod catchers;
pub mod controllers;
pub mod request;
pub mod seo;
pub mod site;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api/users",
            routes![
                controllers::users::sign_in,
                controllers::users::sign_up,
                controllers::users::logs,
                controllers::users::sign_out
            ],
        ));
        items.push(("/", routes![]));
        items
    };
}
