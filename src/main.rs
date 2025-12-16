use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000";

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("SuperM API running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}