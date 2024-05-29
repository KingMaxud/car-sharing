use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn start_rent(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - start_rent", "HANDLER");

    let now = Utc::now();

    let started_request = UpdateOrderDb {
        start_rent_time: Option::from(now.naive_utc()),
        end_rent_time: None,
        status: Option::from("processing".to_string()),
        paid: None,
        updated_at: Option::from(now.naive_utc()),
    };

    let started_rent = orders_service::update(&pool, order_id, started_request)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(started_rent))
}
