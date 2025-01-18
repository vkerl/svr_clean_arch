use super::resp::ErrResp;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("system internal error")]
    InternalError,
}

use self::ApiError::*;

impl ApiError {
    pub fn status_code(&self) -> u32 {
        match self {
            InternalError => 1
        }
    }
}

impl Into<ErrResp> for ApiError {
    fn into(self) -> ErrResp {
        ErrResp { code:  self.status_code(), msg: format!("{}", self), data: None }
    }
}