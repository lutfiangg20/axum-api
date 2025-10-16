use sqlx::{Pool, Postgres};

use crate::products::model::{CreateProduct, Product};

pub struct ProductRepository {
    pub pool: Pool<Postgres>,
}

impl ProductRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
                select p.id, p.name, p.price, p.stock, c.name as category 
                from products p
                join categories c on p.category_id = c.id
                "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(products)
    }

    pub async fn save(&self, create: CreateProduct) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            insert into products (
                name,
                price,
                stock,
                category_id
                ) 
            values ($1,$2,$3,$4)"#,
            create.name,
            create.price,
            create.stock,
            create.category_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
