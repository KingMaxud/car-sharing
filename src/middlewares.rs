use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum::response::Redirect;
use tower_cookies::Cookies;
use tracing::log::debug;

use crate::handlers::auth::{SESSION_TOKEN, UserData};
use crate::handlers::DbPool;
use crate::infra::services::sessions_service;
use crate::models::AuthError;

pub async fn inject_user_data(
    State(pool): State<DbPool>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    debug!("->> {:<12} - inject_user_data", "MIDDLEWARE");

    if let Some(session_token) = cookies.get(SESSION_TOKEN).map(|c| c.value().to_string()) {
        let telegram_id = sessions_service::get_telegram_id_by_token(&pool, session_token)
            .await
            .map_err(AuthError::CarSharingError);

        if let Ok(telegram_id) = telegram_id {
            request.extensions_mut().insert(UserData { telegram_id });
        }
    }

    Ok(next.run(request).await)
}

pub async fn require_auth(req: Request<Body>, next: Next) -> Result<impl IntoResponse, AuthError> {
    debug!("->> {:<12} - require_auth", "MIDDLEWARE");

    if req.extensions().get::<UserData>().is_some() {
        Ok(next.run(req).await)
    } else {
        Ok(Redirect::to("/api/login").into_response())
    }
}
