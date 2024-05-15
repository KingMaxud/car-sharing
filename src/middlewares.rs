use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum::response::Redirect;
use tower_cookies::Cookies;
use tracing::log::debug;

use crate::handlers::auth::{SESSION_TOKEN, UserData};
use crate::handlers::DbPool;
use crate::infra::services::{sessions_service, users_service};
use crate::models::HandlerError;

pub async fn inject_user_data(
    State(pool): State<DbPool>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("->> {:<12} - inject_user_data", "MIDDLEWARE");

    if let Some(session_token) = cookies.get(SESSION_TOKEN).map(|c| c.value().to_string()) {
        let ids = sessions_service::get_ids_by_token(&pool, session_token)
            .await
            .map_err(HandlerError::CarSharingError);

        if let Ok(ids) = ids {
            request.extensions_mut().insert(UserData {
                telegram_id: ids.0,
                user_id: ids.1,
            });
        }
    }

    Ok(next.run(request).await)
}

pub async fn require_auth(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("->> {:<12} - require_auth", "MIDDLEWARE");

    if req.extensions().get::<UserData>().is_some() {
        Ok(next.run(req).await)
    } else {
        Ok(Redirect::to("/api/login").into_response())
    }
}

pub async fn require_admin(
    State(pool): State<DbPool>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("->> {:<12} - require_auth", "MIDDLEWARE");

    if let Some(user_data) = req.extensions().get::<UserData>() {
        let is_admin = users_service::check_if_admin(&pool, user_data.user_id)
            .await
            .map_err(HandlerError::CarSharingError)?;

        if is_admin {
            Ok(next.run(req).await)
        } else {
            Ok(Redirect::to("/api/login").into_response())
        }
    } else {
        Ok(Redirect::to("/api/login").into_response())
    }
}
