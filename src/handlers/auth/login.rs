use axum::{Extension, Json};
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use hex::encode;
use ring::{
    digest,
    hmac::{HMAC_SHA256, Key, sign},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::log::debug;

use crate::config::config;
use crate::handlers::auth::{SESSION_TOKEN, UserData};
use crate::handlers::DbPool;
use crate::infra::Random;
use crate::infra::services::{sessions_service, users_service};
use crate::models::HandlerError;

async fn verify_telegram_hash(
    telegram_response: TelegramLoginResponse,
) -> Result<(), HandlerError> {
    let config = config().await;

    // Generate the secret key using SHA-256 hash of the bot token
    let secret_key = digest::digest(&digest::SHA256, config.bot_token().as_ref())
        .as_ref()
        .to_owned();

    // String to be signed
    let mut data_check_string = String::new();
    let fields = vec![
        ("auth_date", telegram_response.auth_date.to_string()),
        ("first_name", telegram_response.first_name.clone()),
        ("id", telegram_response.id.to_string()),
        ("last_name", telegram_response.last_name.clone()),
        ("photo_url", telegram_response.photo_url.clone()),
        ("username", telegram_response.username.clone()),
    ];

    // Fill the string with values from telegram_response
    for (key, value) in fields {
        if key == "username" {
            data_check_string.push_str(&format!("{}={}", key, value));
        } else {
            data_check_string.push_str(&format!("{}={}\n", key, value));
        }
    }

    let key = Key::new(HMAC_SHA256, secret_key.as_slice());
    let signature_value = sign(&key, data_check_string.as_ref());

    if encode(signature_value) != telegram_response.hash {
        Err(HandlerError::TelegramHashProblem)
    } else {
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct TelegramLoginResponse {
    auth_date: i32,
    first_name: String,
    hash: String,
    id: i32,
    last_name: String,
    photo_url: String,
    username: String,
}

impl Clone for TelegramLoginResponse {
    fn clone(&self) -> Self {
        Self {
            auth_date: self.auth_date,
            first_name: self.first_name.clone(),
            hash: self.hash.clone(),
            id: self.id,
            last_name: self.last_name.clone(),
            photo_url: self.photo_url.clone(),
            username: self.username.clone(),
        }
    }
}

pub async fn login(
    cookies: Cookies,
    Extension(user_data): Extension<Option<UserData>>,
    Extension(random): Extension<Random>,
    State(pool): State<DbPool>,
    Json(login_res): Json<TelegramLoginResponse>,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("->> {:<12} - login", "HANDLER");

    // create new user if not exist
    let user_id = users_service::insert_if_not_exists(&pool, login_res.id.clone()).await?;

    // check if already authenticated
    if user_data.is_some() {
        return Ok(Redirect::to("/"));
    }

    verify_telegram_hash(login_res.clone()).await?;

    let session_token = sessions_service::new_session(&pool, user_id, random)
        .await
        .map_err(HandlerError::CarSharingError)?;

    let cookie_session = session_token.into_cookie_value();

    let mut cookie = Cookie::new(SESSION_TOKEN, cookie_session);

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_secure(true);
    cookies.add(cookie);

    Ok(Redirect::to("/api"))
}
