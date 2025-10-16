use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::users::{AppState, model::CreateUser};

pub async fn get_all(state: State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.user_service.get_all_users().await.unwrap();
    Json(users)
}

pub async fn create(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    match state.user_service.create_user(payload).await {
        Ok(_) => (StatusCode::CREATED, "User has been created".to_string()),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
