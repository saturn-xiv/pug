// http://www.robotstxt.org/
#[get("/robots.txt")]
pub fn robots_txt() -> &'static str {
    r#"
User-agent: *
Disallow: /api/
"#
}
