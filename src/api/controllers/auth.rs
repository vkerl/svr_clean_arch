use actix_web::web;

use crate::{api::{dto::auth::{LoginDto, TokenDto}, resp::{ApiResult, ErrResp}}, app_state::AppState};

pub async fn login(data: web::Data<AppState>, params: web::Json<LoginDto>) -> ApiResult<TokenDto> {
    let result = data.auth_service.login(params.0.into()).await.map_err(|e| -> ErrResp { e.into() })?;
    let token_dto: TokenDto = result.into();
    Ok(web::Json(token_dto.into()))
}