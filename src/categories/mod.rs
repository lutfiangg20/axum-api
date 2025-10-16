use std::sync::Arc;

use axum::{Router, routing::get};

pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

pub struct AppState {
    category_service: service::CategoryService,
}

pub async fn new(pool: sqlx::PgPool) -> Router {
    let category_repository = repository::CategoryRepository::new(pool);
    let category_service = service::CategoryService::new(category_repository);
    let shared_state = Arc::new(AppState { category_service });

    Router::new().route(
        "/",
        get(controller::get_all)
            .post(controller::create)
            .with_state(shared_state),
    )
}
