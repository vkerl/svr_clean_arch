use async_trait::async_trait;

use crate::entities::{errors::CommonError, models::user::{NewUser, User}};

#[derive(Debug)]
pub struct AuthWechatQueryParams {
    openid: String,
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create(&self, params: NewUser) -> Result<(), CommonError>;
    async fn get_by_openid(&self, openid: &str) -> Result<Option<User>, CommonError>;
    async fn get_by_uid(&self, uid: u32) -> Result<Option<User>, CommonError>;
}