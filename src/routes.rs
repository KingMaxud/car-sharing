use axum::{Router, routing::get};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tower_cookies::CookieManagerLayer;

use crate::AppState;
use crate::handlers::auth::login::login;
use crate::handlers::auth::logout::logout;
use crate::handlers::auth::UserData;

#[derive(Debug, Clone)]
pub struct Message {
    // Some shared state for your application
    message: String,
}

pub fn app_router(state: AppState) -> Router {
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    let user_data: Option<UserData> = None;

    let message = Message {
        message: "from the extension!".to_string(),
    };

    Router::new()
        .nest("/api/post", auth_routes())
        .layer(CookieManagerLayer::new())
        .with_state(state)
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/logout", get(logout))
}
