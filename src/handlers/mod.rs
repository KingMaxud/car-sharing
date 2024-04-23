use bb8::PooledConnection;
use diesel::result::{Error as DieselError, Error::QueryBuilderError};
use diesel_async::{
    pg::AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

pub mod cars;
pub mod auth;
pub mod orders;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn get_conn(
    pool: &DbPool,
) -> Result<PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>, DieselError> {
    pool.get().await.map_err(|e| QueryBuilderError(e.into()))
}
