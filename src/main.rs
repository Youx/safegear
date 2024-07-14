pub mod models;
pub mod schema;

use axum::{routing::get, Router};
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use diesel_migrations::MigrationHarness;
use std::error::Error;

#[cfg(debug_assertions)]
fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // In debug mode, reset all migrations every time, because
    // we may be debugging migrations.
    let _ = connection.revert_all_migrations(MIGRATIONS)?;
    let _ = connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
#[cfg(not(debug_assertions))]
fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // In release mode, only run pending migrations
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

fn run_migrations_url(url: String) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        let mut conn = diesel::pg::PgConnection::establish(&url).unwrap();
        run_migrations(&mut conn)
            .map_err(|e| {
                tracing::error!("Failed to apply migrations: {e}");
                e
            })
            .unwrap();
    })
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db_url: String) -> shuttle_axum::ShuttleAxum {
    run_migrations_url(db_url.clone()).await.unwrap();

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
