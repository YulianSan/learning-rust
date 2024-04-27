use axum::Router;
use dotenv::dotenv;
mod databases;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = databases::postgres_connection::start_connection().await;
    let app = Router::new()
        .nest("/api", services::users::services::routes())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7979").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
