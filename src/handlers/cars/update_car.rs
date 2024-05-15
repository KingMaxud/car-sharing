use axum::extract::{Path, State};
use axum::Json;
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::cars::{CarResponse, UpdateCarRequest};
use crate::handlers::DbPool;
use crate::infra::services::cars_service;
use crate::models::HandlerError;

pub async fn update_car(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(updated_car): Json<UpdateCarRequest>,
) -> Result<Json<CarResponse>, HandlerError> {
    debug!("->> {:<12} - update_car", "HANDLER");

    let car = cars_service::update(&pool, id, updated_car)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok(Json(car))
}
