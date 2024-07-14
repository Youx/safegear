use crate::db::DbPool;

#[derive(Clone)]
pub struct Application {
    pub database: DbPool,
}
