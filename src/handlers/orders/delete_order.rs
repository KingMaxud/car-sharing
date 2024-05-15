use axum::extract::{Path, State};
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn delete_order(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<String, HandlerError> {
    debug!("->> {:<12} - delete_order", "HANDLER");

    let res = orders_service::delete(&pool, order_id)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(res)
}
