use diesel::{
    Queryable, Selectable,
};
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = oauth2_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {}

pub async fn _gg(pool: &deadpool_diesel::postgres::Pool) -> Result<(), InfraError> {}

