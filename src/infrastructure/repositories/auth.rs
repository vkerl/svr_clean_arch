use std::sync::Arc;
use sqlx::PgPool;
use async_trait::async_trait;

use crate::entities::{errors::CommonError, models::user::{NewUser, User}, repositories::auth::AuthRepository};

pub struct AuthPostgresRepository {
    pub db_pool: Arc<PgPool>,
}

impl AuthPostgresRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuthRepository for AuthPostgresRepository {
    async fn create(&self, params: NewUser) -> Result<(), CommonError> {
        sqlx::query(
            "INSERT INTO users (uid, openid, session_key, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(params.uid as i32)  // 确保使用 i32 类型
        .bind(&params.openid)
        .bind(&params.session_key)
        .bind(&params.created_at)  // 使用引用来绑定 DateTime
        .bind(&params.updated_at)
        .execute(&*self.db_pool)
        .await
        .map_err(CommonError::DatabaseError)?;

        Ok(())
    }

    async fn get_by_openid(&self, openid: &str) -> Result<Option<User>, CommonError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, uid, openid, session_key, created_at, updated_at FROM users WHERE openid = $1"
        )
        .bind(openid)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(CommonError::DatabaseError)?;

        Ok(user)
    }

    async fn get_by_uid(&self, uid: u32) -> Result<Option<User>, CommonError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, uid, openid, session_key, created_at, updated_at FROM users WHERE uid = $1"
        )
        .bind(uid as i32)  // 转换为 i32
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(CommonError::DatabaseError)?;

        Ok(user)
    }
}