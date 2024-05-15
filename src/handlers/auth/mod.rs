use uuid::Uuid;

pub mod login;
pub mod logout;

#[derive(Clone, Debug)]
pub struct UserData {
    pub telegram_id: i32,
    pub user_id: Uuid,
}

pub const SESSION_TOKEN: &str = "session-token";
