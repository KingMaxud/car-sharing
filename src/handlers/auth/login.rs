use axum::{
    extract::State,
    Json,
    response::{IntoResponse, Redirect},
};
use hex::encode;
use ring::{
    digest,
    hmac::{HMAC_SHA256, Key, sign},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::AppState;
use crate::config::config;
use crate::models::AuthError;

async fn verify_telegram_hash(telegram_response: TelegramLoginResponse) -> Result<(), AuthError> {
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
        Err(AuthError::TelegramHashProblem)
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
// TODO login_res + cookies NOT WORKING

pub async fn login(
    State(state): State<AppState>,
    Json(login_res): Json<TelegramLoginResponse>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AuthError> {
    Ok(Redirect::to("/"))
}

async fn logingg(
    State(state): State<AppState>,
    Json(login_res): Json<TelegramLoginResponse>,
    // Extension(random): Extension<Random>,
    // Extension(user_data): Extension<Option<UserData>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AuthError> {
    verify_telegram_hash(login_res.clone()).await.unwrap();
    // Check if the user with the corresponding Telegram ID already exists in your database. If not — create a new one
    // let user_id = users_service::insert_if_not_exists(&state.pool, login_res.id.clone())
    //     .await
    //     .map_err(AuthError::CarSharingError)?;
    //
    // // Generate a unique session identifier
    // let session_token = sessions_service::new_session(&state.pool, user_id, random)
    //     .await
    //     .map_err(AuthError::CarSharingError)?;

    // if user_data.is_some() {
    //     // check if already authenticated
    //     return Ok(Redirect::to("/"));
    // }

    // let headers = axum::response::AppendHeaders([(
    //     axum::http::header::SET_COOKIE,
    //     "session_token=".to_owned() + "zalupa" + "; path=/; httponly; secure; samesite=strict",
    // )]);

    let mut cookie = Cookie::new("session_token", "zalupa");

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    Ok(Redirect::to("/"))
}
