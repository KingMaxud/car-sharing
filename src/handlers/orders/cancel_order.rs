use axum::{Extension, Json};
use axum::extract::{Path, State};
use chrono::Utc;
use tracing::log::debug;
use uuid::Uuid;

use crate::error::CarSharingError;
use crate::handlers::auth::UserData;
use crate::handlers::DbPool;
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::services::orders_service;
use crate::models::HandlerError;

pub async fn cancel_order(
    State(pool): State<DbPool>,
    Extension(user_data): Extension<UserData>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<String>, HandlerError> {
    debug!("->> {:<12} - cancel_order", "HANDLER");

    let user_id_of_order = orders_service::get(&pool, order_id)
        .await
        .map_err(HandlerError::CarSharingError)?
        .user_id;

    if user_id_of_order == user_data.user_id {
        let now = Utc::now();

        let cancel_request = UpdateOrderDb {
            start_rent_time: None,
            end_rent_time: None,
            status: Option::from("cancelled".to_string()),
            paid: None,
            updated_at: Option::from(now.naive_utc()),
        };

        let cancelled_order = orders_service::update(&pool, order_id, cancel_request)
            .await
            .map_err(HandlerError::CarSharingError);

        match cancelled_order {
            Ok(_) => Ok(Json("Your order was cancelled!".to_string())),
            Err(err) => Err(err),
        }
    } else {
        Err(HandlerError::OwnershipError)
    }
}
