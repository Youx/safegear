use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use axum::{extract::State, Json};
use diesel::{ExpressionMethods as _, QueryDsl as _};
use diesel_async::RunQueryDsl as _;
use jwt_simple::{algorithms::MACLike, claims::Claims};

use crate::{
    api::{ApiClaims, ApiError},
    models::user::User,
    schema::users,
};

use super::{ApiResult, Application};

#[derive(serde::Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct LoginUser {
    login: String,
    password: String,
}

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct UserToken {
    jwt_token: String,
}

pub async fn handler(
    state: State<Application>,
    Json(data): Json<LoginUser>,
) -> ApiResult<Json<UserToken>> {
    let mut conn = state.database.get().await?;
    let user = users::table
        .filter(users::login.eq(&data.login))
        .filter(users::is_active.eq(true))
        .get_result::<User>(&mut conn)
        .await?;
    let parsed_hash = PasswordHash::new(&user.password).map_err(|e| {
        tracing::error!(
            "Invalid password hash for user `{}` in database: {e}",
            &data.login
        );
        ApiError::Database(diesel::result::Error::NotFound)
    })?;
    Argon2::default()
        .verify_password(data.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Database(diesel::result::Error::NotFound))?;

    let claims =
        Claims::with_custom_claims(ApiClaims::from(user), Duration::from_secs(60 * 60).into());
    let jwt_token = state.jwt_secret.authenticate(claims).map_err(|e| {
        tracing::error!("Failed to authenticate claims: {e}");
        ApiError::Database(diesel::result::Error::NotFound)
    })?;
    Ok(Json(UserToken { jwt_token }))
}
