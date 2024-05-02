use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use tower_cookies::{Cookie, Cookies};

use crate::handlers::auth::SESSION_TOKEN;
use crate::handlers::DbPool;
use crate::infra::services::sessions_service;
use crate::models::AuthError;

pub async fn logout(
    cookies: Cookies,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, AuthError> {
    let session_token = cookies.get(SESSION_TOKEN).map(|c| c.value().to_string());

    if let Some(session_token) = session_token {
        sessions_service::delete_session(&pool, session_token)
            .await
            .map_err(AuthError::CarSharingError)?;
    }

    cookies.remove(Cookie::from(Cookie::build(SESSION_TOKEN).path("/")));
    Ok(Redirect::to("/"))
}
