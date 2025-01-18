use serde::{Deserialize, Serialize};

use crate::api::resp::Resp;


#[derive(Debug, Deserialize)]
pub struct LoginDto {
    js_code: String,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub uid: u32,
}