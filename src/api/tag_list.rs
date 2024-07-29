use axum::{extract::State, Json};
use diesel_async::RunQueryDsl as _;

use super::{ApiResult, Application, AuthenticatedUser, NoPermission};
use crate::{models::tag::Tag as TagModel, schema};

#[derive(schemars::JsonSchema, serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct Tag {
    /// Id of the tag
    id: i64,
    /// Name of the tag
    name: String,
    /// Color of the tag
    color: String,
}

impl From<TagModel> for Tag {
    fn from(value: TagModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            color: value.color,
        }
    }
}

pub async fn handler(
    _auth: AuthenticatedUser<NoPermission>,
    state: State<Application>,
) -> ApiResult<Json<Vec<Tag>>> {
    let mut conn = state.database.get().await?;
    let tags = schema::tags::table
        .get_results::<TagModel>(&mut conn)
        .await?;

    Ok(Json(
        tags.into_iter()
            .map(|tag_model| tag_model.into())
            .collect::<Vec<Tag>>(),
    ))
}
