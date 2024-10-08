#[macro_use]
pub mod db;
pub mod api;
pub mod models;
#[cfg(debug_assertions)]
pub mod provisioning;
pub mod schema;

use axum::{
    routing::{delete, get, post},
    Router,
};
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use api::{
    item_create, item_details, item_inspect, item_list, r#static, tag_create, tag_delete, tag_list,
    user_create, user_delete, user_list, user_login, Application,
};
use db::create_pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use diesel_migrations::MigrationHarness;
use jwt_simple::algorithms::HS256Key;
use std::error::Error;

#[cfg(debug_assertions)]
fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // In debug mode, reset all migrations every time, because
    // we may be debugging migrations.
    tracing::info!("Reverting all migrations");
    let _ = connection.revert_all_migrations(MIGRATIONS)?;
    tracing::info!("Applying all migrations");
    let _ = connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
#[cfg(not(debug_assertions))]
fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // In release mode, only run pending migrations
    tracing::info!("Applying new migrations");
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
    let db_pool = create_pool(&db_url);
    run_migrations_url(db_url.clone()).await.unwrap();

    #[cfg(debug_assertions)]
    {
        provisioning::provision(&mut db_pool.get().await.unwrap())
            .await
            .unwrap();
        r#static::list_assets();
    }

    let application = Application {
        database: db_pool,
        // Automatically generate a new key on startup,
        // ensuring all users get logged out.
        jwt_secret: HS256Key::generate(),
    };
    let router = Router::new()
        .fallback(get(r#static::static_handler))
        .route("/", get(r#static::index_handler))
        .route("/api/items", get(item_list::handler))
        .route("/api/items", post(item_create::handler))
        .route("/api/items/:id", get(item_details::handler))
        .route("/api/items/:id/events/inspect", post(item_inspect::handler))
        .route("/api/tags", get(tag_list::handler))
        .route("/api/tags", post(tag_create::handler))
        .route("/api/tags/:id", delete(tag_delete::handler))
        .route("/api/users", get(user_list::handler))
        .route("/api/users", post(user_create::handler))
        .route("/api/users/:id", delete(user_delete::handler))
        .route("/api/users/login", post(user_login::handler))
        .with_state(application);

    Ok(router.into())
}
