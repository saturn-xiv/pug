use std::net::SocketAddr;
use std::ops::Deref;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use super::super::super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, jwt::Jwt, orm::Database,
};
use super::super::{
    auth::{
        log::{Dao as LogDao, Item as Log},
        user::Dao as UserDao,
    },
    request::CurrentUser,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub uid: String,
    pub act: Action,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Action {
    SignIn,
    Confirm,
    Unlock,
    ResetPassword,
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignIn {
    #[validate(length(min = "1"))]
    pub id: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[post("/sign-in", format = "json", data = "<form>")]
pub fn sign_in(
    db: Database,
    jwt: State<Jwt>,
    remote: SocketAddr,
    form: Json<SignIn>,
) -> Result<JsonValue> {
    form.validate()?;
    let ip = remote.ip();
    let db = db.deref();
    let user = UserDao::by_email_or_nick_name(db, &form.id)?;

    if let Err(e) = user.auth::<Sodium>(&form.password) {
        LogDao::add(db, &user.id, &ip, "Sign in failed")?;
        return Err(e);
    }
    user.available()?;
    UserDao::sign_in(db, &user.id, &ip)?;
    LogDao::add(db, &user.id, &ip, "Sign in success")?;
    let token = jwt.sum(
        None,
        &Token {
            uid: user.uid,
            act: Action::SignIn,
        },
    )?;
    Ok(json!({ "token": token }))
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
    // TODO
    Ok(json!({}))
}

#[derive(Debug, Validate, Deserialize)]
pub struct Email {
    #[validate(email)]
    pub email: String,
}

#[post("/confirm", format = "json", data = "<form>")]
pub fn confirm(form: Json<Email>) -> Result<JsonValue> {
    form.validate()?;
    // TODO
    Ok(json!({}))
}

#[put("/confirm/<token>")]
pub fn confirm_token(token: String) -> Result<JsonValue> {
    // TODO
    debug!("confirm {}", token);
    Ok(json!({}))
}

#[post("/unlock", format = "json", data = "<form>")]
pub fn unlock(form: Json<Email>) -> Result<JsonValue> {
    form.validate()?;
    // TODO
    Ok(json!({}))
}

#[put("/unlock/<token>")]
pub fn unlock_token(token: String) -> Result<JsonValue> {
    // TODO
    debug!("unlock {}", token);
    Ok(json!({}))
}

#[post("/forgot-password", format = "json", data = "<form>")]
pub fn forgot_password(form: Json<Email>) -> Result<JsonValue> {
    form.validate()?;
    // TODO
    Ok(json!({}))
}

#[derive(Debug, Validate, Deserialize)]
pub struct ResetPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[post("/reset-password", format = "json", data = "<form>")]
pub fn reset_password(form: Json<ResetPassword>) -> Result<JsonValue> {
    form.validate()?;
    // TODO
    Ok(json!({}))
}

#[get("/logs")]
pub fn logs() -> Result<Json<Vec<Log>>> {
    // TODO
    Ok(Json(Vec::new()))
}

#[get("/profile")]
pub fn get_profile() -> Result<Json<()>> {
    // TODO
    Ok(Json(()))
}

#[post("/profile")]
pub fn post_profile() -> Result<Json<()>> {
    // TODO
    Ok(Json(()))
}

#[derive(Debug, Validate, Deserialize)]
pub struct ChangePassword {
    #[validate(length(min = "1"))]
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

#[post("/change-password", format = "json", data = "<form>")]
pub fn change_password(
    db: Database,
    form: Json<ChangePassword>,
    user: CurrentUser,
    remote: SocketAddr,
) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    let ip = remote.ip();
    let user = UserDao::by_id(db, &user.id)?;
    user.auth::<Sodium>(&form.current_password)?;
    UserDao::password::<Sodium>(db, &user.id, &form.new_password)?;
    LogDao::add(db, &user.id, &ip, "Change password")?;
    Ok(Json(()))
}

#[delete("/sign-out")]
pub fn sign_out(db: Database, user: CurrentUser, remote: SocketAddr) -> Result<Json<()>> {
    let db = db.deref();
    let ip = remote.ip();
    LogDao::add(db, &user.id, &ip, "Sign out")?;
    Ok(Json(()))
}
