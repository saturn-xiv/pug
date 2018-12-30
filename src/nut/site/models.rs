use chrono::NaiveDateTime;

use super::super::super::orm::ID;

#[derive(Queryable, Serialize)]
pub struct LeaveWord {
    pub id: ID,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
}
