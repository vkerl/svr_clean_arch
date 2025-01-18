use std::fmt::Display;

use serde::Serialize;
use serde_json::to_string;
use actix_web::{body::BoxBody, error, http::header::ContentType, web, HttpRequest, HttpResponse, Responder};

#[derive(Debug, Serialize)]
pub struct Resp<T> {
    pub code: u32,
    pub msg: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Default for Resp<T> {
    fn default() -> Self {
        Self { code: 0, msg: "succ".into(), data: None }
    }
}

impl<T> std::convert::From<T> for Resp<T> {
    fn from(v: T) -> Self {
        let mut resp = Resp::default();
        resp.data = Some(v);
        resp
    }
}

impl<T> Responder for Resp<T> where T : serde::Serialize {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

// 实现错误响应
impl Display for Resp<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = to_string(&self).unwrap();
        f.write_str(&format!("Response: {}", display_str))
    }
}

impl error::ResponseError for Resp<()> {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }
}

pub type ErrResp = Resp<()>;

pub type ApiResult<T> = Result<web::Json<Resp<T>>, ErrResp>;