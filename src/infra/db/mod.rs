use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::log::debug;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn run_migrations(url: &str) {
    use diesel::prelude::*;

    debug!("Running migrations");

    let mut conn = diesel::pg::PgConnection::establish(url).expect("Failed to connect to database");
    // &mut impl MigrationHarness<diesel::pg::Pg>
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
