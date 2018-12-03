use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use super::super::super::errors::Result;
use super::super::auth::log;

#[derive(Debug, Validate, Deserialize)]
pub struct SignIn {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[post("/sign-in", format = "json", data = "<form>")]
pub fn sign_in(form: Json<SignIn>) -> Result<JsonValue> {
    form.validate()?;
    Ok(json!({}))
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignUp {
    #[validate(length(min = "1", max = "32"))]
    pub real_name: String,
    #[validate(length(min = "1", max = "32"))]
    pub nick_name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[post("/sign-up", format = "json", data = "<form>")]
pub fn sign_up(form: Json<SignUp>) -> Result<JsonValue> {
    form.validate()?;
    Ok(json!({}))
}

#[get("/logs")]
pub fn logs() -> Result<Json<Vec<log::Item>>> {
    Ok(Json(Vec::new()))
}

#[delete("/sign-out")]
pub fn sign_out() -> Result<Json<()>> {
    Ok(Json(()))
}
