use axum::{extract::State, Json};
use diesel_async::RunQueryDsl as _;

use super::{ApiResult, Application};
use crate::{models::user::User as UserModel, schema};

#[derive(schemars::JsonSchema, serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct UserWithPermissions {
    /// Id of the user
    id: i64,
    /// Login of the user
    login: String,
    /// Whether the user is enabled or disabled
    is_active: bool,
    /// User has permission to manage other users
    perm_users: bool,
    /// User has permission to manage tags
    perm_tags: bool,
    /// User has permission to manage items
    perm_items: bool,
    /// User has permission to inspect items
    perm_action_inspect: bool,
    /// User has permission to lend items
    perm_action_lend: bool,
}

impl From<UserModel> for UserWithPermissions {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            login: value.login,
            is_active: value.is_active,
            perm_users: value.perm_users,
            perm_tags: value.perm_tags,
            perm_items: value.perm_items,
            perm_action_inspect: value.perm_action_inspect,
            perm_action_lend: value.perm_action_lend,
        }
    }
}

pub async fn handler(state: State<Application>) -> ApiResult<Json<Vec<UserWithPermissions>>> {
    let mut conn = state.database.get().await?;
    let users = schema::users::table
        .get_results::<UserModel>(&mut conn)
        .await?;

    Ok(Json(
        users
            .into_iter()
            .map(|user_model| user_model.into())
            .collect::<Vec<UserWithPermissions>>(),
    ))
}
