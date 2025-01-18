use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn db_pool(db_name: &str) -> PgPool {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = format!("{}/{}", database_url, db_name);

    PgPoolOptions::new().max_connections(5).connect(&database).await.unwrap()
}