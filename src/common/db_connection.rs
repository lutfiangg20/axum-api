use dotenvy::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn new() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&url)
        .await?;

    Ok(pool)
}
