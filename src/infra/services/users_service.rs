use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, OptionalExtension, Queryable, QueryDsl, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::error::{CarSharingError, Result};
use crate::infra::db::schema::users;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub status: String,
    pub telegram_id: i32,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub telegram_id: i32,
}

pub async fn insert_if_not_exists(
    pool: &deadpool_diesel::postgres::Pool,
    telegram_id: i32,
) -> Result<Uuid> {
    use crate::infra::db::schema::users::dsl::users;

    debug!("->> {:<12} - insert_if_not_exists", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(|err| CarSharingError::from(err))?;

    let existing_user = conn
        .interact(move |conn| {
            users
                .filter(users::telegram_id.eq(telegram_id))
                .first::<UserDb>(conn)
                .optional()
        })
        .await
        .map_err(|err| CarSharingError::from(err))?
        .map_err(|err| CarSharingError::from(err))?;

    // Create new user if necessary
    let user_id = match existing_user {
        Some(user) => user.id, // User already exists, use their ID
        None => {
            let id = conn
                .interact(move |conn| {
                    let new_user = NewUserDb { telegram_id }; // Create a new user struct
                    diesel::insert_into(users)
                        .values(&new_user)
                        .returning(users::id)
                        .get_result(conn)
                })
                .await
                .map_err(CarSharingError::DatabaseNotFound)?
                .map_err(CarSharingError::DatabaseNotFound)?;

            id
        }
    };

    Ok(user_id)
}
