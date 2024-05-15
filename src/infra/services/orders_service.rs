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
use crate::infra::db::schema::orders;
use crate::infra::db::schema::orders::dsl::*;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = orders)]
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
#[diesel(table_name = orders)]
pub struct NewOrderDb {
    pub user_id: Uuid,
    pub car_id: Uuid,
}

#[derive(Deserialize)]
pub struct OrdersFilter {
    pub user_id: Option<Uuid>,
}

#[derive(AsChangeset)]
#[diesel(table_name = orders)]
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

pub async fn delete(pool: &DbPool, order_id: Uuid) -> Result<String> {
    debug!("->> {:<12} - delete", "INFRASTRUCTURE");

    let conn = &mut get_conn(pool).await?;

    diesel::delete(orders.filter(id.eq(order_id)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok("Order was successfully deleted!".to_string())
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
