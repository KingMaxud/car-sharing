use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use tower_cookies::{Cookie, Cookies};

use crate::AppState;
use crate::handlers::auth::SESSION_TOKEN;
use crate::infra::services::sessions_service;
use crate::models::AuthError;

pub async fn logout(
    cookies: Cookies,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AuthError> {
    let session_token = cookies.get(SESSION_TOKEN).map(|c| c.value().to_string());

    if let Some(session_token) = session_token {
        sessions_service::delete_session(&state.pool, session_token)
            .await
            .map_err(AuthError::CarSharingError)?;
    }

    cookies.remove(Cookie::from(SESSION_TOKEN));
    Ok(Redirect::to("/"))
}
