use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::categories::{AppState, model::CreateCategory};

pub async fn get_all(state: State<Arc<AppState>>) -> impl IntoResponse {
    let categories = state.category_service.get_all_users().await.unwrap();
    Json(categories)
}

pub async fn create(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateCategory>,
) -> impl IntoResponse {
    match state.category_service.create_user(payload).await {
        Ok(_) => (StatusCode::CREATED, "Category has been created".to_string()),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
