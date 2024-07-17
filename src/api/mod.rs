use axum::{http::StatusCode, response::IntoResponse};

use crate::db::DbPool;

pub mod item_create;
pub mod item_details;
pub mod item_list;
pub mod r#static;
pub mod tag_create;
pub mod tag_list;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Pool error: {0}")]
    Pool(#[from] diesel_async::pooled_connection::deadpool::PoolError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, format!("Unknown element"))
            }
            ApiError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            ),
            ApiError::Pool(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Pool error: {e}"),
            ),
        }
        .into_response()
    }
}

pub(super) type ApiResult<T> = Result<T, ApiError>;

#[derive(Clone)]
pub struct Application {
    pub database: DbPool,
}
