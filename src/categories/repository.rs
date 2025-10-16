use sqlx::{Pool, Postgres};

use crate::categories::model::{CreateCategory, Product};

pub struct CategoryRepository {
    pub pool: Pool<Postgres>,
}

impl CategoryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Product>, sqlx::Error> {
        let categories = sqlx::query_as!(Product, "select * from categories")
            .fetch_all(&self.pool)
            .await?;
        Ok(categories)
    }

    pub async fn save(&self, create: CreateCategory) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("insert into categories (name) values ($1)", create.name,)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
