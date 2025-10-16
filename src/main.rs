use axum::Router;
mod categories;
mod common;
mod users;

#[tokio::main]
async fn main() {
    let db = common::db_connection::new().await.unwrap();
    let app = Router::new()
        .nest("/users", users::new(db.clone()).await)
        .nest("/categories", categories::new(db.clone()).await);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
