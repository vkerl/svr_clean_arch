
use actix_web::web;

use crate::api::{dto::auth::UserDto, resp::ApiResult};


pub fn login() -> ApiResult<UserDto> {
    let d = UserDto{ uid: 101 };
    Ok(web::Json(d.into()))
}