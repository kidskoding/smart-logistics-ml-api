use std::env;
use dotenv::dotenv;
use sqlx::PgPool;

pub async fn connect() -> sqlx::Result<PgPool> {
    dotenv().ok();

    let url = env::var("DATABASE_URL")
        .expect("database url env variable has not been set!");
    let pool: PgPool = PgPool::connect(&url).await?;

    Ok(pool)
}