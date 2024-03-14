use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tracing::log::debug;
use crate::AppState;

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new().route("/", get(root)).fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running!"
}

async fn handler_404() -> impl IntoResponse {
    debug!("->> {:<12} - handler_404", "HANDLER");

    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
