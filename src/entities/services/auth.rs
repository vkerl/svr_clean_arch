use async_trait::async_trait;

use crate::entities::{errors::CommonError, models::auth::{AuthLoginParams, AuthLoginResult}};

#[async_trait]
pub trait AuthService: 'static + Sync + Send {
    async fn login(&self, params: AuthLoginParams) -> Result<AuthLoginResult, CommonError>;
}