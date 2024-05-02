pub mod login;
pub mod logout;

#[derive(Clone, Debug)]
pub struct UserData {
    pub telegram_id: i32,
}

pub const SESSION_TOKEN: &str = "session-token";
