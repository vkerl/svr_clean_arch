use async_trait::async_trait;

use crate::entities::models::auth::AuthLoginParams;

#[async_trait]
pub trait AuthService: 'static + Sync + Send {
    async fn login(&self, params: AuthLoginParams);
}