use axum::extract::{Query, State};
use axum::Json;
use tracing::log::debug;

use crate::handlers::cars::CarResponse;
use crate::handlers::DbPool;
use crate::infra::services::{cars_service, cars_service::CarsFilter};
use crate::models::HandlerError;
use crate::models::HandlerError::CarSharingError;

pub async fn list_cars(
    State(pool): State<DbPool>,
    Query(params): Query<CarsFilter>,
) -> Result<Json<Vec<CarResponse>>, HandlerError> {
    debug!("->> {:<12} - list_cars", "HANDLER");

    let cars = cars_service::get_all(&pool, params)
        .await
        .map_err(|err| CarSharingError(err))?;

    Ok(Json(cars))
}
