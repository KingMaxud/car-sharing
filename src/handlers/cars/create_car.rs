use axum::{extract::State, Json};
use tracing::log::debug;

use crate::handlers::cars::{CarResponse, CreateCarRequest};
use crate::handlers::DbPool;
use crate::infra::services::cars_service;
use crate::models::HandlerError;

pub async fn create_car(
    State(pool): State<DbPool>,
    Json(new_car): Json<CreateCarRequest>,
) -> Result<Json<CarResponse>, HandlerError> {
    debug!("->> {:<12} - create_car", "HANDLER");

    let new_car_db = cars_service::NewCarDb {
        name: new_car.name,
        hourly_rate: new_car.hourly_rate,
        daily_rate: new_car.daily_rate,
        weekly_rate: new_car.weekly_rate,
        photos: new_car.photos,
        status: new_car.status,
    };

    let created_car = cars_service::insert(&pool, new_car_db).await?;

    Ok(Json(created_car))
}
