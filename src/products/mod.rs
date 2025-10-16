use std::sync::Arc;

use axum::{Router, routing::get};

pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

pub struct AppState {
    product_service: service::ProductsService,
}

pub async fn new(pool: sqlx::PgPool) -> Router {
    let product_repository = repository::ProductRepository::new(pool);
    let product_service = service::ProductsService::new(product_repository);
    let shared_state = Arc::new(AppState {
        product_service: product_service,
    });

    Router::new().route(
        "/",
        get(controller::get_all)
            .post(controller::create)
            .with_state(shared_state),
    )
}
