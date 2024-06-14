use std::sync::{Arc, Mutex};

use axum::{
    Extension, http::StatusCode, middleware, response::IntoResponse, Router, routing::get,
    routing::post,
};
use axum::routing::{delete, patch};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tower_cookies::CookieManagerLayer;
use tracing::log::debug;

use crate::handlers::auth::login::login;
use crate::handlers::auth::logout::logout;
use crate::handlers::auth::UserData;
use crate::handlers::cars::create_car::create_car;
use crate::handlers::cars::delete_car::delete_car;
use crate::handlers::cars::get_car::get_car;
use crate::handlers::cars::list_cars::list_cars;
use crate::handlers::cars::update_car::update_car;
use crate::handlers::DbPool;
use crate::handlers::orders::accept_order::accept_order;
use crate::handlers::orders::cancel_order::cancel_order;
use crate::handlers::orders::delete_order::delete_order;
use crate::handlers::orders::finish_rent::finish_rent;
use crate::handlers::orders::get_order::get_order;
use crate::handlers::orders::list_orders::list_orders;
use crate::handlers::orders::make_order::make_order;
use crate::handlers::orders::orders_history::orders_history;
use crate::handlers::orders::set_paid::set_paid;
use crate::handlers::orders::start_rent::start_rent;
use crate::infra::db::run_migrations;
use crate::middlewares::{inject_user_data, require_admin, require_auth};

pub async fn app_router(db_url: &str) -> Router {
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    let user_data: Option<UserData> = None;

    run_migrations(db_url);

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    Router::new()
        .route("/", get(root))
        .merge(auth_routes())
        .nest("/cars", cars_routes(pool.clone()))
        .nest("/orders", orders_user_routes())
        .nest("/orders", orders_admin_routes(pool.clone()))
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
        .route("/logout", post(logout))
}

fn cars_routes(pool: DbPool) -> Router<DbPool> {
    Router::new()
        .route("/", post(create_car))
        .route("/:id", get(get_car))
        .route("/:id", patch(update_car))
        .route("/:id", delete(delete_car))
        .route("/", get(list_cars))
        .route_layer(middleware::from_fn_with_state(pool, require_admin))
}

fn orders_user_routes() -> Router<DbPool> {
    Router::new()
        .route("/history", get(orders_history))
        .route("/", post(make_order))
        .route("/cancel/:id", patch(cancel_order))
        .route_layer(middleware::from_fn(require_auth))
}

fn orders_admin_routes(pool: DbPool) -> Router<DbPool> {
    Router::new()
        .route("/:id", get(get_order))
        .route("/", get(list_orders))
        .route("/accept/:id", patch(accept_order))
        .route("/:id", delete(delete_order))
        .route("/finish/:id", patch(finish_rent))
        .route("/set_paid/:id", patch(set_paid))
        .route("/start/:id", patch(start_rent))
        .route_layer(middleware::from_fn_with_state(pool, require_admin))
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
