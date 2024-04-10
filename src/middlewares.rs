use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::TypedHeader;
use headers::Cookie;

use crate::AppState;
use crate::handlers::auth::UserData;
use crate::infra::services::sessions_service;
use crate::models::AuthError;

pub async fn inject_user_data(
    State(state): State<AppState>,
    cookie: Option<TypedHeader<Cookie>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    if let Some(cookie) = cookie {
        if let Some(session_token) = cookie.get("session_token") {
            let telegram_id = sessions_service::get_telegram_id_by_token(&state.pool, String::from(session_token))
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
