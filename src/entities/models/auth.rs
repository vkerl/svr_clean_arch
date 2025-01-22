use serde::{Deserialize, Serialize};


pub struct AuthLoginParams {
    pub js_code: String,
    pub app_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtPayload {
    pub uid: i32,
    pub exp: i64,
}

pub struct AuthLoginResult {
    pub uid: u32,
    pub token: String,
}