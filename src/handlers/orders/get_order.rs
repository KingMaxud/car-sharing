use axum::extract::{Path, State};
use axum::Json;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::handlers::orders::OrderResponse;
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn get_order(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - get_order", "HANDLER");

    let order = orders_service::get(&pool, order_id)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(order))
}
