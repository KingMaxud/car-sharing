use axum::extract::{Path, State};
use tracing::log::debug;
use uuid::Uuid;

use crate::handlers::DbPool;
use crate::infra::services::cars_service;
use crate::models::HandlerError;

pub async fn delete_car(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<String, HandlerError> {
    debug!("->> {:<12} - delete_car", "HANDLER");

    cars_service::delete(&pool, id)
        .await
        .map_err(HandlerError::CarSharingError)?;

    Ok("Car was successfully deleted!".to_string())
}
