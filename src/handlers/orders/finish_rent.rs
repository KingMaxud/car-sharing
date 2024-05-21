use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn finish_rent(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - finish_rent", "HANDLER");

    let now = Utc::now();

    let finished_request = UpdateOrderDb {
        start_rent_time: None,
        end_rent_time: Option::from(now.naive_utc()),
        status: Option::from("finished".to_string()),
        paid: None,
        updated_at: Option::from(now.naive_utc()),
    };

    let finished_rent = orders_service::update(&pool, order_id, finished_request)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(finished_rent))
}
