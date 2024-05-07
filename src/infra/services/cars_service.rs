use chrono::NaiveDateTime;
use diesel::{
    AsChangeset, ExpressionMethods, Insertable, Queryable, QueryDsl, Selectable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::error::{CarSharingError, Result};
use crate::handlers::{DbPool, get_conn};
use crate::handlers::cars::{CarResponse, UpdateCarRequest};
use crate::infra::db::schema::cars as cars_table;
use crate::infra::db::schema::cars::dsl::*;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = cars_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CarDb {
    pub id: Uuid,
    pub name: String,
    pub hourly_rate: i32,
    pub daily_rate: i32,
    pub weekly_rate: i32,
    pub photos: Option<Vec<Option<String>>>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = cars_table)]
pub struct NewCarDb {
    pub name: String,
    pub hourly_rate: i32,
    pub daily_rate: i32,
    pub weekly_rate: i32,
    pub photos: Option<Vec<Option<String>>>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CarsFilter {
    status: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = cars_table)]
struct UpdateCarChangeset {
    name: Option<String>,
    hourly_rate: Option<i32>,
    daily_rate: Option<i32>,
    weekly_rate: Option<i32>,
    status: Option<String>,
}

pub async fn insert(pool: &DbPool, new_car: NewCarDb) -> Result<CarResponse> {
    debug!("->> {:<12} - insert", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let res = diesel::insert_into(cars)
        .values(&new_car)
        .get_result::<CarDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(CarResponse::from(res))
}

pub async fn get(pool: &DbPool, car_id: Uuid) -> Result<CarResponse> {
    debug!("->> {:<12} - get", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let res = cars
        .filter(id.eq(car_id))
        .select(CarDb::as_select())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(CarResponse::from(res))
}

pub async fn get_all(pool: &DbPool, _filter: CarsFilter) -> Result<Vec<CarResponse>> {
    debug!("->> {:<12} - get_all", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let mut query = cars.into_boxed::<diesel::pg::Pg>();

    let res = query
        .select(CarDb::as_select())
        .load::<CarDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    let car_responses: Vec<CarResponse> = res.into_iter().map(CarResponse::from).collect();

    Ok(car_responses)
}

pub async fn delete(pool: &DbPool, car_id: Uuid) -> Result<()> {
    debug!("->> {:<12} - delete", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    diesel::delete(cars.filter(id.eq(car_id)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(())
}

pub async fn update(
    pool: &DbPool,
    car_id: Uuid,
    updated_car: UpdateCarRequest,
) -> Result<CarResponse> {
    debug!("->> {:<12} - update", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let changeset = UpdateCarChangeset {
        name: updated_car.name,
        hourly_rate: updated_car.hourly_rate,
        daily_rate: updated_car.daily_rate,
        weekly_rate: updated_car.weekly_rate,
        status: updated_car.status,
    };

    let res = diesel::update(cars.find(car_id))
        .set(&changeset)
        .returning(CarDb::as_returning())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(CarResponse::from(res))
}
