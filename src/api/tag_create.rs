use axum::{extract::State, Json};
use diesel_async::RunQueryDsl as _;

use crate::{
    models::tag::{InsertTag as InsertTagModel, Tag as TagModel},
    schema::tags,
};

use super::{tag_list::Tag, ApiResult, Application, AuthenticatedUser, ManageTags};

#[derive(serde::Deserialize, schemars::JsonSchema, ts_rs::TS)]
#[ts(export)]
pub struct CreateTag {
    /// Name of the tag to create
    name: String,
    /// Color of the tag
    color: String,
}

pub async fn handler(
    _auth: AuthenticatedUser<ManageTags>,
    state: State<Application>,
    Json(data): Json<CreateTag>,
) -> ApiResult<Json<Tag>> {
    let mut conn = state.database.get().await?;
    let tag = diesel::insert_into(tags::table)
        .values(InsertTagModel {
            name: data.name,
            color: data.color,
        })
        .returning(tags::all_columns)
        .get_result::<TagModel>(&mut conn)
        .await?;

    Ok(Json(tag.into()))
}
