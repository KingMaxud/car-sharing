use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infra::services::orders_service::OrderDb;

// User:
pub mod cancel_order;
pub mod make_order;
pub mod orders_history;
// Admin
pub mod accept_order;
pub mod delete_order;
pub mod finish_rent;
pub mod get_order;
pub mod list_orders;
pub mod set_paid;
pub mod start_rent;

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    id: Uuid,
    user_id: Uuid,
    car_id: Uuid,
    start_rent_time: Option<NaiveDateTime>,
    end_rent_time: Option<NaiveDateTime>,
    status: String,
    paid: bool,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl From<OrderDb> for OrderResponse {
    fn from(order_db: OrderDb) -> Self {
        OrderResponse {
            id: order_db.id,
            user_id: order_db.user_id,
            car_id: order_db.car_id,
            start_rent_time: order_db.start_rent_time,
            end_rent_time: order_db.end_rent_time,
            status: order_db.status,
            paid: order_db.paid,
            created_at: order_db.created_at,
            updated_at: order_db.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MakeOrderRequest {
    car_id: Uuid,
}

#[derive(Debug)]
pub struct UpdateOrderDb {
    pub start_rent_time: Option<NaiveDateTime>,
    pub end_rent_time: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub paid: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}
