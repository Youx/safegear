use axum::{http::StatusCode, response::IntoResponse};
use jwt_simple::algorithms::HS256Key;

use crate::{db::DbPool, models::user::User};

pub mod item_create;
pub mod item_details;
pub mod item_list;
pub mod r#static;
pub mod tag_create;
pub mod tag_delete;
pub mod tag_list;
pub mod user_list;
pub mod user_login;

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
    pub jwt_secret: HS256Key,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApiClaims {
    login: String,
    perm_users: bool,
    perm_tags: bool,
    perm_items: bool,
    perm_action_inspect: bool,
    perm_action_lend: bool,
}

impl From<User> for ApiClaims {
    fn from(value: User) -> Self {
        Self {
            login: value.login,
            perm_users: value.perm_users,
            perm_action_lend: value.perm_action_lend,
            perm_action_inspect: value.perm_action_inspect,
            perm_tags: value.perm_tags,
            perm_items: value.perm_items,
        }
    }
}
