use diesel::{
    Associations, ExpressionMethods, Insertable, Queryable, QueryDsl, Selectable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::error::{CarSharingError, Result};
use crate::handlers::{DbPool, get_conn};
use crate::infra::db::schema::{sessions as sessions_table, users};
use crate::infra::db::schema::sessions::dsl::*;
use crate::infra::Random;
use crate::infra::services::users_service::UserDb;
use crate::models::session_token::SessionToken;

#[derive(Debug, Serialize, Queryable, Selectable, Associations)]
#[diesel(belongs_to(UserDb, foreign_key = user_id))]
#[diesel(table_name = sessions_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionDb {
    pub session_token: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = sessions_table)]
pub struct NewSessionDb {
    session_token: Vec<u8>,
    user_id: Uuid,
}

pub async fn new_session(pool: &DbPool, user_id_req: Uuid, random: Random) -> Result<SessionToken> {
    debug!("->> {:<12} - new_session", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Generate new token
    let session_token_generated = SessionToken::generate_new(random);

    // Create NewSession to insert into db
    let new_session = NewSessionDb {
        session_token: session_token_generated.into_database_value(),
        user_id: user_id_req,
    };

    diesel::insert_into(sessions)
        .values(new_session)
        .returning(SessionDb::as_returning())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(session_token_generated)
}

pub async fn get_ids_by_token(pool: &DbPool, token: String) -> Result<((i32, Uuid))> {
    debug!("->> {:<12} - get_telegram_id_by_token", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Convert String to Vec<u8>
    let session_token_bytes = token.parse::<u128>()?.to_le_bytes().to_vec();

    let user_db = sessions
        .filter(session_token.eq(session_token_bytes))
        .inner_join(users::table)
        .select(UserDb::as_select())
        .first::<UserDb>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok((user_db.telegram_id, user_db.id))
}

pub async fn delete_session(pool: &DbPool, token: String) -> Result<()> {
    use crate::infra::db::schema::sessions::dsl::*;

    debug!("->> {:<12} - delete_session", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Convert String to Vec<u8>
    let session_token_bytes = token.parse::<u128>()?.to_le_bytes().to_vec();

    diesel::delete(sessions.filter(session_token.eq(session_token_bytes)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
    use rand_chacha::ChaCha8Rng;
    use rand_core::{OsRng, RngCore, SeedableRng};
    use serial_test::serial;

    use crate::infra::services::users_service::insert_if_not_exists;

    use super::*;

    async fn create_connection_pool() -> DbPool {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            "postgres://postgres:postgres@localhost/car-sharing-tests",
        );
        bb8::Pool::builder().build(manager).await.unwrap()
    }

    async fn get_first_session(pool: &DbPool) -> SessionDb {
        let conn = &mut get_conn(pool).await.unwrap();

        sessions
            .first::<SessionDb>(conn)
            .await
            .map_err(|err| CarSharingError::from(err))
            .expect("Can't find a session")
    }

    #[tokio::test]
    #[serial]
    async fn test_01_clear_sessions_database() {
        let pool = create_connection_pool().await;

        let conn = &mut get_conn(&pool).await.unwrap();

        diesel::delete(sessions.filter(session_token.is_not_null()))
            .execute(conn)
            .await
            .map_err(|err| CarSharingError::from(err))
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_02_new_session() {
        let pool = create_connection_pool().await;

        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

        let user_id_res = insert_if_not_exists(&pool, 443621429)
            .await
            .expect("Failed to insert user or retrieve existing ID");

        assert!(
            !new_session(&pool, user_id_res, Arc::new(Mutex::new(random)),)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_03_get_ids_by_token() {
        let pool = create_connection_pool().await;

        let session_token_res = get_first_session(&pool).await;

        let mut arr = [0u8; 16];

        arr.copy_from_slice(&session_token_res.session_token);

        let session_token_string = u128::from_le_bytes(arr).to_string();

        assert!(!get_ids_by_token(&pool, session_token_string).await.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_04_delete_session() {
        let pool = create_connection_pool().await;

        let session_token_res = get_first_session(&pool).await;

        let mut arr = [0u8; 16];

        arr.copy_from_slice(&session_token_res.session_token);

        let session_token_string = u128::from_le_bytes(arr).to_string();

        assert!(!delete_session(&pool, session_token_string).await.is_err());
    }
}
