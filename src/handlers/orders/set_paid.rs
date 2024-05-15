use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn set_paid(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - set_paid", "HANDLER");

    let now = Utc::now();

    let set_paid_request = UpdateOrderDb {
        start_rent_time: None,
        end_rent_time: None,
        status: None,
        paid: Option::from(true),
        updated_at: Option::from(now.naive_utc()),
    };

    let paid_order = orders_service::update(&pool, order_id, set_paid_request)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(paid_order))
}
