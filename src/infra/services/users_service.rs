use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, OptionalExtension, Queryable, QueryDsl, Selectable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::config::config;
use crate::error::{CarSharingError, Result};
use crate::handlers::{DbPool, get_conn};
use crate::infra::db::schema::users as users_table;
use crate::infra::db::schema::users::dsl::*;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub status: String,
    pub telegram_id: i32,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users_table)]
pub struct NewUserDb {
    pub telegram_id: i32,
}

pub async fn insert_if_not_exists(pool: &DbPool, telegram_id_req: i32) -> Result<Uuid> {
    debug!("->> {:<12} - insert_if_not_exists", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Get existing user if exists
    let existing_user = users
        .filter(telegram_id.eq(telegram_id_req))
        .first::<UserDb>(conn)
        .await
        .optional()
        .map_err(|err| CarSharingError::from(err))?;

    // Create new user if necessary
    let user_id = match existing_user {
        Some(user) => user.id, // User already exists, use their ID
        None => {
            let new_user = NewUserDb {
                telegram_id: telegram_id_req,
            }; // Create a new user struct
            diesel::insert_into(users)
                .values(&new_user)
                .returning(id)
                .get_result(conn)
                .await
                .map_err(|err| CarSharingError::from(err))?
        }
    };

    Ok(user_id)
}

pub async fn check_if_admin(pool: &DbPool, user_id_req: Uuid) -> Result<bool> {
    debug!("->> {:<12} - insert_if_not_exists", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    let config = config().await;

    let admin_ids: Vec<i32> = config
        .admin_ids()
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid integer"))
        .collect();

    let user_db = users
        .filter(id.eq(user_id_req))
        .first::<UserDb>(conn)
        .await
        .optional()
        .map_err(|err| CarSharingError::from(err))?;

    match user_db {
        Some(user_db) => {
            if admin_ids.contains(&user_db.telegram_id) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        None => Err(CarSharingError::DatabaseNotFound),
    }
}

#[cfg(test)]
mod tests {
    use diesel::sql_query;
    use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

    use crate::config::config;

    use super::*;

    async fn create_connection_pool() -> DbPool {
        let config = config().await;

        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(config.db_url());
        bb8::Pool::builder().build(manager).await.unwrap()
    }

    #[tokio::test]
    async fn health_checker() {
        let pool = create_connection_pool().await;

        let conn = &mut get_conn(&pool).await.unwrap();

        let res = sql_query("SELECT 1").execute(conn).await;
        assert!(!res.is_err());
    }

    #[tokio::test]
    async fn test_insert_if_not_exists() {
        let pool = create_connection_pool().await;

        insert_if_not_exists(&pool, 443621429)
            .await
            .expect("Failed to insert user or retrieve existing ID");
    }
}
