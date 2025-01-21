use sqlx::Error;

#[derive(thiserror::Error, Debug)]
pub enum CommonError {
    #[error("system internal error")]
    InternalError,

    #[error("database error: {0}")]
    DatabaseError(#[from] Error),
}

use self::CommonError::*;

impl CommonError {
    pub fn status_code(&self) -> u32 {
        match self {
            InternalError => 1,
            DatabaseError(_) => 2,
        }
    }
}