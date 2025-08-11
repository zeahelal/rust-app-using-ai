use axum::{routing::get, Router};

pub fn health_router() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}

pub fn api_router() -> Router {
    Router::new().route("/api/hello", get(|| async { "hello" }))
}
