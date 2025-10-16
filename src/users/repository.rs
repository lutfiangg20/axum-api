use sqlx::{Pool, Postgres};

use crate::users::model::{CreateUser, User};

pub struct UserRepository {
    pub pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(User, "select * from users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn save(&self, create: CreateUser) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "insert into users (name,email) values ($1,$2)",
            create.name,
            create.email
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
