use async_trait::async_trait;
use crate::entities::repositories::auth::AuthRepository;
use crate::entities::services::auth::AuthService;
use crate::entities::models::auth::AuthLoginParams;
use std::sync::Arc;

pub struct AuthServiceImpl {
    pub repo: Arc<dyn AuthRepository>,
}

impl AuthServiceImpl {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, params: AuthLoginParams) {
        // TODO: 实现登录逻辑
    }
}