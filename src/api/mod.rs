use std::marker::PhantomData;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::JWTClaims,
};
use serde_json::json;
use tokio::task::JoinError;

use crate::{
    db::DbPool,
    models::{event::EventData, user::User},
};

pub mod item_create;
pub mod item_details;
pub mod item_inspect;
pub mod item_list;
pub mod r#static;
pub mod tag_create;
pub mod tag_delete;
pub mod tag_list;
pub mod user_create;
pub mod user_delete;
pub mod user_list;
pub mod user_login;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Pool error: {0}")]
    Pool(#[from] diesel_async::pooled_connection::deadpool::PoolError),
    #[error("Cannot delete yourself")]
    CannotDeleteSelf,
    #[error("Error hashing password: {0}")]
    PasswordHash(String),
    #[error("Error joining task: {0}")]
    JoinError(#[from] JoinError),
    #[error("Transitioning from event {0:?} to {1:?} is not allowed")]
    InvalidTransition(Option<EventData>, EventData),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        match self {
            ApiError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, format!("Unknown element"))
            }
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::Pool(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::CannotDeleteSelf => (StatusCode::BAD_REQUEST, message),
            ApiError::PasswordHash(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::JoinError(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::InvalidTransition(..) => (StatusCode::BAD_REQUEST, message),
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

// error types for auth errors
#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingPermission,
}

pub struct AuthenticatedUser<P: ClaimPermission> {
    claims: ApiClaims,
    phantom: PhantomData<P>,
}
pub trait ClaimPermission {
    fn check(claims: &ApiClaims) -> bool;
}
impl<P> TryFrom<JWTClaims<ApiClaims>> for AuthenticatedUser<P>
where
    P: ClaimPermission + Default,
{
    type Error = AuthError;

    fn try_from(value: JWTClaims<ApiClaims>) -> Result<Self, Self::Error> {
        if P::check(&value.custom) {
            Ok(AuthenticatedUser {
                claims: value.custom,
                phantom: PhantomData::default(),
            })
        } else {
            Err(AuthError::MissingPermission)
        }
    }
}

#[async_trait]
impl<S, P> FromRequestParts<S> for AuthenticatedUser<P>
where
    P: ClaimPermission + Default,
    S: Send + Sync,
    Application: FromRef<S>,
    AuthenticatedUser<P>: TryFrom<JWTClaims<ApiClaims>, Error = AuthError>,
{
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app = Application::from_ref(&state);
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data: JWTClaims<ApiClaims> = app
            .jwt_secret
            .verify_token(bearer.token(), None)
            .map_err(|_| AuthError::InvalidToken)?;

        token_data.try_into()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::MissingPermission => (StatusCode::FORBIDDEN, "Missing permission"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

macro_rules! permissions {
    (NoPermission => true, $($t:ident => $field:ident),* $(,)?) => {
        $(
            #[derive(Default)]
            pub struct $t;
            impl $crate::api::ClaimPermission for $t {
                fn check(c: &$crate::api::ApiClaims) -> bool {
                    c.$field
                }
            }
        )*

        #[derive(Default)]
        pub struct NoPermission;
        impl $crate::api::ClaimPermission for NoPermission {
            fn check(_c: &$crate::api::ApiClaims) -> bool {
                true
            }
        }
    };
}

permissions!(
    NoPermission => true,
    ManageItems => perm_items,
    ManageUsers => perm_users,
    ManageTags => perm_tags,
    LendItems => perm_action_lend,
    InspectItems => perm_action_inspect
);
