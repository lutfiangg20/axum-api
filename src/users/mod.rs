use std::sync::Arc;

use axum::{Router, routing::get};

pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

pub struct AppState {
    user_service: service::UserService,
}

pub async fn new(pool: sqlx::PgPool) -> Router {
    let user_repository = repository::UserRepository::new(pool);
    let user_service = service::UserService::new(user_repository);
    let shared_state = Arc::new(AppState { user_service });

    Router::new().route(
        "/",
        get(controller::get_all)
            .post(controller::create)
            .with_state(shared_state),
    )
}
