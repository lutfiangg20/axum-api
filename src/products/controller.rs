use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::products::{AppState, model::CreateProduct};

pub async fn get_all(state: State<Arc<AppState>>) -> impl IntoResponse {
    let products = state.product_service.get_all_products().await.unwrap();
    Json(products)
}

pub async fn create(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    match state.product_service.create_product(payload).await {
        Ok(_) => (StatusCode::CREATED, "Product has been added".to_string()),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
