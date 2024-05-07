use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infra::services::cars_service::CarDb;

pub mod create_car;
pub mod delete_car;
pub mod get_car;
pub mod list_cars;
pub mod update_car;

#[derive(Debug, Serialize)]
pub struct CarResponse {
    id: Uuid,
    name: String,
    hourly_rate: i32,
    daily_rate: i32,
    weekly_rate: i32,
    photos: Option<Vec<Option<String>>>,
    status: String,
    created_at: NaiveDateTime,
}

impl From<CarDb> for CarResponse {
    fn from(car_db: CarDb) -> Self {
        CarResponse {
            id: car_db.id,
            name: car_db.name,
            hourly_rate: car_db.hourly_rate,
            daily_rate: car_db.daily_rate,
            weekly_rate: car_db.weekly_rate,
            photos: car_db.photos,
            status: car_db.status,
            created_at: car_db.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCarRequest {
    name: String,
    hourly_rate: i32,
    daily_rate: i32,
    weekly_rate: i32,
    photos: Option<Vec<Option<String>>>,
    status: String,
}
#[derive(Debug, Deserialize)]
pub struct UpdateCarRequest {
    pub name: Option<String>,
    pub hourly_rate: Option<i32>,
    pub daily_rate: Option<i32>,
    pub weekly_rate: Option<i32>,
    pub status: Option<String>,
}
