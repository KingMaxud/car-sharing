use axum::extract::{Path, State};
use axum::Json;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::cars::CarResponse;
use crate::handlers::DbPool;
use crate::infra::services::cars_service;
use crate::models::HandlerError;

pub async fn get_car(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<CarResponse>, HandlerError> {
    debug!("->> {:<12} - get_car", "HANDLER");

    let car = cars_service::get(&pool, id)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(car))
}
