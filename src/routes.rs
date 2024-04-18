use std::sync::{Arc, Mutex};

use axum::{
    Extension, http::StatusCode, response::IntoResponse, Router, routing::get, routing::post,
};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tower_cookies::CookieManagerLayer;
use tracing::log::debug;

use crate::AppState;
use crate::handlers::auth::login::login;
use crate::handlers::auth::logout::logout;
use crate::handlers::auth::UserData;

pub fn app_router(state: AppState) -> Router {
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    let user_data: Option<UserData> = None;

    Router::new()
        .route("/", get(root))
        .merge(auth_routes())
        .layer(CookieManagerLayer::new())
        .layer(Extension(user_data))
        .layer(Extension(Arc::new(Mutex::new(random))))
        .with_state(state)
        .fallback(handler_404)
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
}

async fn root() -> &'static str {
    "Server is running!" // Return a simple message indicating the server is running
}

async fn handler_404() -> impl IntoResponse {
    debug!("->> {:<12} - handler_404", "HANDLER");

    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
