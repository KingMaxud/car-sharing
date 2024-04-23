use diesel::{
    Associations, ExpressionMethods, Insertable, Queryable, QueryDsl, Selectable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::error::{CarSharingError, Result};
use crate::handlers::{DbPool, get_conn};
use crate::infra::{Random, services::users_service::UserDb};
use crate::infra::db::schema::{sessions, users};
use crate::models::session_token::SessionToken;

#[derive(Serialize, Queryable, Selectable, Associations)]
#[diesel(belongs_to(UserDb, foreign_key = user_id))]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionDb {
    pub session_token: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    session_token: Vec<u8>,
    user_id: Uuid,
}

pub async fn new_session(pool: &DbPool, user_id: Uuid, random: Random) -> Result<SessionToken> {
    debug!("->> {:<12} - new_session", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    let session_token = SessionToken::generate_new(random);

    let new_session = NewSession {
        session_token: session_token.into_database_value(),
        user_id,
    };

    diesel::insert_into(sessions::table)
        .values(new_session)
        .returning(SessionDb::as_returning())
        .get_result(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    // TODO this works, but further — BUGGED

    Ok(session_token)
}

pub async fn get_telegram_id_by_token(pool: &DbPool, session_token: String) -> Result<i32> {
    debug!("->> {:<12} - get_telegram_id_by_token", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Convert String to Vec<u8>
    let session_token_bytes = session_token.as_bytes().to_vec();

    let telegram_id = sessions::table
        .filter(sessions::session_token.eq(session_token_bytes))
        .inner_join(users::table)
        .select(users::telegram_id)
        .first::<i32>(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(telegram_id)
}

pub async fn delete_session(pool: &DbPool, session_token: String) -> Result<()> {
    // use crate::infra::db::schema::sessions::dsl;

    debug!("->> {:<12} - delete_session", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = &mut get_conn(pool).await?;

    // Convert String to Vec<u8>
    let session_token_bytes = session_token.as_bytes().to_vec();

    diesel::delete(sessions::table.filter(sessions::session_token.eq(session_token_bytes)))
        .execute(conn)
        .await
        .map_err(|err| CarSharingError::from(err))?;

    Ok(())
}
