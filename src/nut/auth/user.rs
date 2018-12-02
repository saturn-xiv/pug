use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use md5;
use uuid::Uuid;

use super::super::super::{
    crypto::Encryptor,
    errors::Result,
    orm::{Connection, ID},
};
use super::schema::users;

#[derive(Debug)]
pub enum Type {
    Google,
    Facebook,
    Line,
    Github,
    WeChat,
    Email,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Google => fmt.write_str("google"),
            Type::Facebook => fmt.write_str("facebook"),
            Type::Github => fmt.write_str("github"),
            Type::WeChat => fmt.write_str("wechat"),
            Type::Line => fmt.write_str("line"),
            Type::Email => fmt.write_str("email"),
        }
    }
}

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: ID,
    pub real_name: String,
    pub nick_name: String,
    pub email: String,
    pub password: Option<Vec<u8>>,
    pub uid: String,
    pub provider_type: String,
    pub provider_id: String,
    pub logo: String,
    pub sign_in_count: i64,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct New<'a> {
    pub real_name: &'a str,
    pub nick_name: &'a str,
    pub email: &'a str,
    pub password: Option<&'a [u8]>,
    pub uid: &'a str,
    pub provider_type: &'a str,
    pub provider_id: &'a str,
    pub logo: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn sign_in(&self, id: &ID, ip: &String) -> Result<()>;
    fn sign_up<T: Encryptor>(
        &self,
        real_name: &String,
        nick_name: &String,
        email: &String,
        password: &String,
    ) -> Result<()>;
    fn lock(&self, id: &ID, on: bool) -> Result<()>;
    fn confirm(&self, id: &ID) -> Result<()>;
    fn count(&self) -> Result<i64>;
    fn is_email_exist(&self, email: &String) -> Result<bool>;
    fn is_nick_name_exist(&self, nick_name: &String) -> Result<bool>;
    fn password<T: Encryptor>(&self, id: &ID, password: &String) -> Result<()>;
}

impl Dao for Connection {
    fn sign_in(&self, id: &ID, ip: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        let (current_sign_in_at, current_sign_in_ip, sign_in_count) = users::dsl::users
            .select((
                users::dsl::current_sign_in_at,
                users::dsl::current_sign_in_ip,
                users::dsl::sign_in_count,
            ))
            .filter(users::dsl::id.eq(id))
            .first::<(Option<NaiveDateTime>, Option<String>, i64)>(self)?;
        update(it)
            .set((
                users::dsl::current_sign_in_at.eq(&now),
                users::dsl::current_sign_in_ip.eq(ip),
                users::dsl::last_sign_in_at.eq(&current_sign_in_at),
                users::dsl::last_sign_in_ip.eq(&current_sign_in_ip),
                users::dsl::sign_in_count.eq(&(sign_in_count + 1)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn sign_up<T: Encryptor>(
        &self,
        real_name: &String,
        nick_name: &String,
        email: &String,
        password: &String,
    ) -> Result<()> {
        insert_into(users::dsl::users)
            .values(&New {
                real_name: real_name,
                nick_name: nick_name,
                email: email,
                password: Some(&T::sum(password.as_bytes())?),
                provider_type: &format!("{}", Type::Email),
                provider_id: email,
                logo: &format!(
                    "https://www.gravatar.com/avatar/{}.jpg",
                    gravatar_hash(email)
                ),
                uid: &Uuid::new_v4().to_string(),
                updated_at: &Utc::now().naive_utc(),
            })
            .execute(self)?;
        Ok(())
    }

    fn lock(&self, id: &ID, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&if on { Some(now) } else { None }),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn confirm(&self, id: &ID) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::confirmed_at.eq(&Some(now)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn count(&self) -> Result<i64> {
        let cnt: i64 = users::dsl::users.count().get_result(self)?;
        Ok(cnt)
    }

    fn is_email_exist(&self, email: &String) -> Result<bool> {
        let cnt: i64 = users::dsl::users
            .filter(users::dsl::email.eq(email))
            .count()
            .get_result(self)?;
        Ok(cnt > 0)
    }

    fn is_nick_name_exist(&self, nick_name: &String) -> Result<bool> {
        let cnt: i64 = users::dsl::users
            .filter(users::dsl::nick_name.eq(nick_name))
            .count()
            .get_result(self)?;
        Ok(cnt > 0)
    }

    fn password<T: Encryptor>(&self, id: &ID, password: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let password = T::sum(password.as_bytes())?;
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::password.eq(&Some(password)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}

// https://en.gravatar.com/site/implement/hash/
pub fn gravatar_hash<S: AsRef<str>>(email: &S) -> String {
    format!(
        "{:x}",
        md5::compute(email.as_ref().to_lowercase().trim().as_bytes())
    )
}
