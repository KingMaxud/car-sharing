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
use crate::handlers::orders::{OrderResponse, UpdateOrderDb};
use crate::infra::db::schema::orders as orders_table;
use crate::infra::db::schema::orders::dsl::*;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = orders_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub car_id: Uuid,
    pub start_rent_time: Option<NaiveDateTime>,
    pub end_rent_time: Option<NaiveDateTime>,
    pub status: String,
    pub paid: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = orders_table)]
pub struct NewOrderDb {
    pub user_id: Uuid,
    pub car_id: Uuid,
}

#[derive(Deserialize)]
pub struct OrdersFilter {
    pub user_id: Option<Uuid>,
}

#[derive(AsChangeset)]
#[diesel(table_name = orders_table)]
struct UpdateOrderChangeset {
    start_rent_time: Option<NaiveDateTime>,
    end_rent_time: Option<NaiveDateTime>,
    status: Option<String>,
    paid: Option<bool>,
    updated_at: Option<NaiveDateTime>,
}

pub async fn insert(pool: &DbPool, new_order_db: NewOrderDb) -> Result<OrderResponse> {
    debug!("->> {:<12} - insert", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let res = diesel::insert_into(orders)
        .values(&new_order_db)
        .get_result::<OrderDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(OrderResponse::from(res))
}

pub async fn get(pool: &DbPool, order_id: Uuid) -> Result<OrderResponse> {
    debug!("->> {:<12} - get", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let res = orders
        .filter(id.eq(order_id))
        .select(OrderDb::as_select())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(OrderResponse::from(res))
}

pub async fn get_all(pool: &DbPool, filter: OrdersFilter) -> Result<Vec<OrderResponse>> {
    debug!("->> {:<12} - get_all", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let mut query = orders.into_boxed::<diesel::pg::Pg>();

    if let Some(user_id_from_filter) = filter.user_id {
        query = query.filter(user_id.eq(user_id_from_filter));
    }

    let res = query
        .select(OrderDb::as_select())
        .load::<OrderDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    let list_response = res
        .into_iter()
        .map(OrderResponse::from)
        .collect::<Vec<OrderResponse>>();

    Ok(list_response)
}

pub async fn update(
    pool: &DbPool,
    order_id: Uuid,
    updated_order: UpdateOrderDb,
) -> Result<OrderResponse> {
    debug!("->> {:<12} - update", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    let changeset = UpdateOrderChangeset {
        start_rent_time: updated_order.start_rent_time,
        end_rent_time: updated_order.end_rent_time,
        status: updated_order.status,
        paid: updated_order.paid,
        updated_at: updated_order.updated_at,
    };

    let res = diesel::update(orders.find(order_id))
        .set(&changeset)
        .returning(OrderDb::as_returning())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(OrderResponse::from(res))
}

pub async fn delete(pool: &DbPool, order_id: Uuid) -> Result<String> {
    debug!("->> {:<12} - delete", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    diesel::delete(orders.filter(id.eq(order_id)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok("Order was successfully deleted!".to_string())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
    use serial_test::serial;

    use crate::infra::services::cars_service;
    use crate::infra::services::cars_service::NewCarDb;
    use crate::infra::services::users_service::insert_if_not_exists;

    use super::*;

    async fn create_connection_pool() -> DbPool {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            "postgres://postgres:postgres@localhost/car-sharing-tests",
        );
        bb8::Pool::builder().build(manager).await.unwrap()
    }

    async fn get_first_order(pool: &DbPool) -> OrderDb {
        let conn = &mut get_conn(pool).await.unwrap();

        orders
            .first::<OrderDb>(conn)
            .await
            .map_err(|err| CarSharingError::from(err))
            .expect("Can't find a session")
    }

    #[tokio::test]
    #[serial]
    async fn test_01_clear_orders_database() {
        let pool = create_connection_pool().await;

        let conn = &mut get_conn(&pool).await.unwrap();

        diesel::delete(orders.filter(id.is_not_null()))
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

        let user_id_res = insert_if_not_exists(&pool, 443621429)
            .await
            .expect("Failed to insert user or retrieve existing ID");

        let new_car_db = NewCarDb {
            name: "".to_string(),
            hourly_rate: 0,
            daily_rate: 0,
            weekly_rate: 0,
            photos: Option::from(vec![Option::from("none".to_string())]),
            status: "".to_string(),
        };

        let new_car_res = cars_service::insert(&pool, new_car_db)
            .await
            .expect("Failed to insert car");

        let new_order = NewOrderDb {
            user_id: user_id_res,
            car_id: new_car_res.id,
        };

        assert!(!insert(&pool, new_order).await.is_err())
    }

    #[tokio::test]
    #[serial]
    async fn test_03_get() {
        let pool = create_connection_pool().await;

        let get_order_res = get_first_order(&pool).await;

        assert!(!get(&pool, get_order_res.id).await.is_err())
    }

    #[tokio::test]
    #[serial]
    async fn test_04_get_all() {
        let pool = create_connection_pool().await;

        let orders_filter = OrdersFilter { user_id: None };

        assert!(!get_all(&pool, orders_filter).await.is_err())
    }

    #[tokio::test]
    #[serial]
    async fn test_05_update() {
        let pool = create_connection_pool().await;

        let get_order_res = get_first_order(&pool).await;

        let now = Utc::now();

        let update_order_req = UpdateOrderDb {
            start_rent_time: None,
            end_rent_time: Option::from(now.naive_utc()),
            status: Option::from("finished".to_string()),
            paid: None,
            updated_at: Option::from(now.naive_utc()),
        };

        let res = update(&pool, get_order_res.id, update_order_req)
            .await
            .expect("Failed to update an order");

        assert_eq!("finished".to_string(), res.status)
    }

    #[tokio::test]
    #[serial]
    async fn test_06_delete() {
        let pool = create_connection_pool().await;

        let get_order_res = get_first_order(&pool).await;

        assert!(!delete(&pool, get_order_res.id).await.is_err())
    }
}
