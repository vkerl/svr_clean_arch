use serde::{Deserialize, Serialize};

use crate::entities::models::auth::AuthLoginParams;


#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub js_code: String,
    pub app_name: String,
}

impl Into<AuthLoginParams> for LoginDto {
    fn into(self) -> AuthLoginParams {
        AuthLoginParams {
            js_code: self.js_code,
            app_name: self.app_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub uid: u32,
}