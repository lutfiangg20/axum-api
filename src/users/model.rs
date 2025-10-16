use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
