use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct CarModel {
    pub id: Uuid,
    pub name: String,
    pub hourly_rate: i32,
    pub daily_rate: i32,
    pub weekly_rate: i32,
    pub photos: Option<Vec<Option<String>>>,
    pub status: String,
    pub created_at: NaiveDateTime,
}
