use axum::{routing::get, Router};
use diesel_async::pooled_connection::deadpool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db_pool: deadpool::Pool<diesel_async::AsyncPgConnection>,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
