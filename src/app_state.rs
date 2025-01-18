use std::sync::Arc;

use sqlx::PgPool;


pub struct AppSate {
    
}

impl AppSate {
    pub fn new(db_pool: Arc<PgPool>) {
        
    }
}