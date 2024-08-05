use axum::{
    extract::{Path, State},
    Json,
};
use diesel::QueryDsl as _;
use diesel_async::RunQueryDsl as _;

use super::{ApiError, ApiResult, Application, AuthenticatedUser, ManageUsers};
use crate::{
    models::user::User as UserModel,
    schema::{self, users},
};

pub async fn handler(
    auth: AuthenticatedUser<ManageUsers>,
    state: State<Application>,
    Path(user_id): Path<i64>,
) -> ApiResult<Json<()>> {
    let mut conn = state.database.get().await?;
    let user = schema::users::table
        .find(user_id)
        .get_result::<UserModel>(&mut conn)
        .await?;
    if user.login == auth.claims.login {
        Err(ApiError::CannotDeleteSelf)
    } else {
        diesel::delete(users::table.find(user.id))
            .execute(&mut conn)
            .await?;
        Ok(Json(()))
    }
}
