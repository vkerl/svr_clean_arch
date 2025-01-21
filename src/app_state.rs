use std::sync::Arc;
use sqlx::PgPool;

use crate::entities::services::auth::AuthService;
use crate::infrastructure::repositories::auth_repo::AuthRepository;
use crate::services::auth::AuthServiceImpl;

pub struct AppSate {
    auth_service: Arc<dyn AuthService>,
}

impl AppSate {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        let auth_service = Arc::new(AuthServiceImpl::new(Arc::new(AuthRepository::new(db_pool.clone()))));
        Self { auth_service }
    }
}