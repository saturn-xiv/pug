use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use super::super::super::errors::Result;
use super::super::auth::log;

#[derive(Debug, Validate, Deserialize)]
pub struct SignInFm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[post("/sign-in", format = "json", data = "<form>")]
pub fn sign_in(form: Json<SignInFm>) -> Result<JsonValue> {
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
