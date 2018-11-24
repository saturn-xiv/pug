use askama::Template;

#[derive(Template)]
#[template(path = "bootstrap/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "bootstrap/not-found.html")]
pub struct NotFound<'a> {
    pub title: &'a str,
}
