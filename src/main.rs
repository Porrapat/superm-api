mod products;

use dotenvy::dotenv;
use std::env;
use axum::{
    routing::get,
    Router,
};

use tower_http::{
    services::ServeDir
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(|| async { "Hello, SuperM API!" }))
        .route("/products-list", get(products::products_list))
        .route("/products/id/{id}", get(products::product_by_id))
        .nest_service("/images", ServeDir::new("images"));

    println!("SuperM API running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
