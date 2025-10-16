use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub category: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub category_id: i32,
}
