use std::sync::{Arc, Mutex};

use axum::{
    Extension, http::StatusCode, middleware, response::IntoResponse, Router, routing::get,
    routing::post,
};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tower_cookies::CookieManagerLayer;
use tracing::log::debug;

use crate::handlers::auth::login::login;
use crate::handlers::auth::logout::logout;
use crate::handlers::auth::UserData;
use crate::handlers::DbPool;
use crate::middlewares::inject_user_data;

pub async fn app_router(db_url: &str) -> Router {
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    let user_data: Option<UserData> = None;

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    Router::new()
        .route("/", get(root))
        .merge(auth_routes())
        .layer(Extension(user_data))
        .layer(Extension(Arc::new(Mutex::new(random))))
        .layer(middleware::from_fn_with_state(
            pool.clone(),
            inject_user_data,
        ))
        .layer(CookieManagerLayer::new())
        .with_state(pool)
        .fallback(handler_404)
}

fn auth_routes() -> Router<DbPool> {
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
