use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::TypedHeader;
use headers::Cookie;
use tracing::log::debug;

use crate::handlers::auth::UserData;
use crate::handlers::DbPool;
use crate::infra::services::sessions_service;
use crate::models::AuthError;

pub async fn inject_user_data(
    State(pool): State<&DbPool>,
    cookie: Option<TypedHeader<Cookie>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    debug!("->> {:<12} - inject_user_data", "MIDDLEWARE");

    if let Some(cookie) = cookie {
        if let Some(session_token) = cookie.get("session_token") {
            let telegram_id =
                sessions_service::get_telegram_id_by_token(&pool, String::from(session_token))
                    .await
                    .map_err(AuthError::CarSharingError);

            if let Ok(telegram_id) = telegram_id {
                request
                    .extensions_mut()
                    .insert(Some(UserData { telegram_id }));
            }
        }
    }

    Ok(next.run(request).await)
}
