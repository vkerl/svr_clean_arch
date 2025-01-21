use std::sync::Arc;
use sqlx::PgPool;
pub struct AuthRepository {
    pub db_pool: Arc<PgPool>,
}

impl AuthRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}