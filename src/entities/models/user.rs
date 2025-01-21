use chrono::{DateTime, Utc};
pub struct NewUser {
    pub uid: u32,
    pub openid: String,
    pub session_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct User {
    pub id: u32,
    pub uid: u32,
    pub openid: String,
    pub session_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}