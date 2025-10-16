use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}
