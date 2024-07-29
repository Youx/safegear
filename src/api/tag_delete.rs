use axum::{
    extract::{Path, State},
    Json,
};
use diesel::QueryDsl as _;
use diesel_async::RunQueryDsl as _;

use super::{ApiResult, Application, AuthenticatedUser, ManageTags};
use crate::schema::*;

pub async fn handler(
    _auth: AuthenticatedUser<ManageTags>,
    state: State<Application>,
    Path(tag_id): Path<i64>,
) -> ApiResult<Json<()>> {
    let mut conn = state.database.get().await?;
    Ok(diesel::delete(tags::table.find(tag_id))
        .execute(&mut conn)
        .await
        .map(|_| Json(()))?)
}
