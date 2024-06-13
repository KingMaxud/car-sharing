use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn accept_order(
    State(pool): State<DbPool>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, HandlerError> {
    debug!("->> {:<12} - accept_order", "HANDLER");

    let now = Utc::now();

    let accept_request = UpdateOrderDb {
        start_rent_time: None,
        end_rent_time: None,
        status: Option::from("accepted".to_string()),
        paid: None,
        updated_at: Option::from(now.naive_utc()),
    };

    let accepted_order = orders_service::update(&pool, order_id, accept_request)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(accepted_order))
}
