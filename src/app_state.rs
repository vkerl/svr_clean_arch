use std::sync::Arc;
use sqlx::PgPool;

use crate::entities::services::auth::AuthService;
use crate::infrastructure::repositories::auth::AuthPostgresRepository;
use crate::services::auth::AuthServiceImpl;

pub struct AppState {
    pub auth_service: Arc<dyn AuthService>,
}

impl AppState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        let auth_repo = Arc::new(AuthPostgresRepository::new(db_pool));
        let auth_service = Arc::new(AuthServiceImpl::new(auth_repo));
        Self { auth_service }
    }
}