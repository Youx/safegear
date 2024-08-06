use argon2::{password_hash::SaltString, Argon2, PasswordHasher as _};
use axum::{extract::State, Json};
use diesel_async::RunQueryDsl as _;

use super::{
    user_list::UserWithPermissions, ApiError, ApiResult, Application, AuthenticatedUser,
    ManageUsers,
};
use crate::{
    models::user::{InsertUser, User as UserModel},
    schema::users,
};

#[derive(serde::Deserialize, schemars::JsonSchema, ts_rs::TS)]
#[ts(export)]
pub struct CreateUser {
    login: String,
    password: String,
    is_active: bool,
    perm_users: bool,
    perm_tags: bool,
    perm_items: bool,
    perm_action_inspect: bool,
    perm_action_lend: bool,
}

pub async fn handler(
    _auth: AuthenticatedUser<ManageUsers>,
    state: State<Application>,
    Json(CreateUser {
        login,
        password,
        is_active,
        perm_users,
        perm_items,
        perm_tags,
        perm_action_lend,
        perm_action_inspect,
    }): Json<CreateUser>,
) -> ApiResult<Json<UserWithPermissions>> {
    let password = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut rand::rngs::OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| ApiError::PasswordHash(e.to_string()))?
            .to_string();
        Ok::<_, ApiError>(hash)
    })
    .await??;
    let mut conn = state.database.get().await?;

    Ok(diesel::insert_into(users::table)
        .values(InsertUser {
            login,
            password,
            is_active,
            perm_users,
            perm_tags,
            perm_items,
            perm_action_inspect,
            perm_action_lend,
        })
        .returning(users::all_columns)
        .get_result::<UserModel>(&mut conn)
        .await
        .map(|user| Json(user.into()))?)
}
