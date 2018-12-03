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
                controllers::users::confirm,
                controllers::users::confirm_token,
                controllers::users::forgot_password,
                controllers::users::unlock,
                controllers::users::unlock_token,
                controllers::users::reset_password,
                controllers::users::logs,
                controllers::users::get_profile,
                controllers::users::post_profile,
                controllers::users::change_password,
                controllers::users::sign_out
            ],
        ));
        items.push((
            "/",
            routes![
                seo::robots::txt,
                seo::sitemap::xml_gz,
                seo::rss::atom,
                controllers::home
            ],
        ));
        items
    };
}
