use axum::{Extension, Json};
use axum::extract::State;
use tracing::log::debug;

use crate::handlers::auth::UserData;
use crate::handlers::DbPool;
use crate::handlers::orders::{MakeOrderRequest, OrderResponse};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn make_order(
    State(pool): State<DbPool>,
    Extension(user_data): Extension<UserData>,
    Json(make_order_request): Json<MakeOrderRequest>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - make_order", "HANDLER");

    let new_order_db = orders_service::NewOrderDb {
        user_id: user_data.user_id,
        car_id: make_order_request.car_id,
    };

    let order = orders_service::insert(&pool, new_order_db).await?;

    Ok(Json(order))
}
