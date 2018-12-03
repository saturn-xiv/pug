pub mod admin;
pub mod leave_words;
pub mod locales;
pub mod ueditor;
pub mod users;

#[get("/")]
pub fn home() -> &'static str {
    // TODO
    "home"
}
