use std::env;

use dotenvy::dotenv;
use tokio::sync::OnceCell;

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
    bot_token: String,
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn bot_token(&self) -> &str {
        &self.bot_token
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    // Load environment variables from a .env file
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("0606"))
            .parse::<u16>()
            .unwrap(),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Config {
        server: server_config,
        db: database_config,
        bot_token: env::var("BOT_TOKEN").expect("BOT_TOKEN must be set"),
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
