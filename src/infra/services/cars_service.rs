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

    // Get a database connection from the pool and handle any potential errors
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

    // Get a database connection from the pool and handle any potential errors
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

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Create a query to add filters later
    let mut query = cars.into_boxed::<diesel::pg::Pg>();

    let res = query
        .select(CarDb::as_select())
        .load::<CarDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    // Make Vec<CarResponse> from res
    let list_response = res.into_iter().map(CarResponse::from).collect();

    Ok(list_response)
}

pub async fn update(
    pool: &DbPool,
    car_id: Uuid,
    updated_car: UpdateCarRequest,
) -> Result<CarResponse> {
    debug!("->> {:<12} - update", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
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

pub async fn delete(pool: &DbPool, car_id: Uuid) -> Result<()> {
    debug!("->> {:<12} - delete", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    diesel::delete(cars.filter(id.eq(car_id)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
    use serial_test::serial;

    use super::*;

    async fn create_connection_pool() -> DbPool {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            "postgres://postgres:postgres@localhost/car-sharing-tests",
        );
        bb8::Pool::builder().build(manager).await.unwrap()
    }

    async fn get_first_car(pool: &DbPool) -> CarDb {
        let conn = &mut get_conn(pool).await.unwrap();

        cars.first::<CarDb>(conn)
            .await
            .map_err(|err| CarSharingError::from(err))
            .expect("Can't find a car")
    }

    #[tokio::test]
    #[serial]
    async fn test_01_clear_cars_database() {
        let pool = create_connection_pool().await;

        let conn = &mut get_conn(&pool).await.unwrap();

        diesel::delete(cars.filter(id.is_not_null()))
            .execute(conn)
            .await
            .map_err(|err| CarSharingError::from(err))
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_02_insert() {
        let pool = create_connection_pool().await;

        let conn = &mut get_conn(&pool).await.unwrap();

        let new_car_db = NewCarDb {
            name: "".to_string(),
            hourly_rate: 0,
            daily_rate: 0,
            weekly_rate: 0,
            photos: Option::from(vec![Option::from("none".to_string())]),
            status: "".to_string(),
        };

        assert!(!insert(&pool, new_car_db).await.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_03_get() {
        let pool = create_connection_pool().await;

        let get_car_res = get_first_car(&pool).await;

        assert!(!get(&pool, get_car_res.id).await.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_04_get_all() {
        let pool = create_connection_pool().await;

        let cars_filter = CarsFilter { status: None };

        assert!(!get_all(&pool, cars_filter).await.is_err())
    }

    #[tokio::test]
    #[serial]
    async fn test_05_update() {
        let pool = create_connection_pool().await;

        let get_car_res = get_first_car(&pool).await;

        let update_car_req = UpdateCarRequest {
            name: None,
            hourly_rate: Option::from(5),
            daily_rate: None,
            weekly_rate: None,
            status: None,
        };

        let res = update(&pool, get_car_res.id, update_car_req)
            .await
            .expect("Failed to update a car");

        assert_eq!(res.hourly_rate, 5)
    }

    #[tokio::test]
    #[serial]
    async fn test_06_delete() {
        let pool = create_connection_pool().await;

        let get_car_res = get_first_car(&pool).await;

        assert!(!delete(&pool, get_car_res.id).await.is_err())
    }
}
