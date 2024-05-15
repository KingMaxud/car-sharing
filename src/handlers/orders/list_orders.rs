use axum::extract::{Query, State};
use axum::Json;
use tracing::log::debug;

use crate::handlers::DbPool;
use crate::handlers::orders::OrderResponse;
use crate::infra::services::orders_service;
use crate::infra::services::orders_service::OrdersFilter;
use crate::models::HandlerError;
use crate::models::HandlerError::CarSharingError;

pub async fn list_orders(
    State(pool): State<DbPool>,
    Query(params): Query<OrdersFilter>,
) -> Result<Json<Vec<OrderResponse>>, HandlerError> {
    debug!("->> {:<12} - list_orders", "HANDLER");

    let orders = orders_service::get_all(&pool, params)
        .await
        .map_err(|err| CarSharingError(err))?;

    Ok(Json(orders))
}
