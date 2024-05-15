use axum::{Extension, Json};
use axum::extract::State;
use tracing::log::debug;

use crate::error::CarSharingError;
use crate::handlers::auth::UserData;
use crate::handlers::DbPool;
use crate::handlers::orders::OrderResponse;
use crate::infra::services::orders_service;
use crate::infra::services::orders_service::OrdersFilter;
use crate::models::HandlerError;

pub async fn orders_history(
    State(pool): State<DbPool>,
    Extension(user_data): Extension<UserData>,
) -> Result<Json<Vec<OrderResponse>>, HandlerError> {
    debug!("->> {:<12} - orders_history", "HANDLER");

    let filter = OrdersFilter {
        user_id: Option::from(user_data.user_id),
    };

    let orders = orders_service::get_all(&pool, filter)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(Json(orders))
}
