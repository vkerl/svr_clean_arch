use crate::entities::repositories::auth::AuthRepository;
use std::sync::Arc;

pub struct AuthServiceImpl {
    pub repo: Arc<dyn AuthRepository>,
}

impl AuthServiceImpl {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }
}