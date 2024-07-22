use axum::{extract::State, Json};
use diesel::{BelongingToDsl as _, GroupedBy as _};
use diesel_async::RunQueryDsl as _;

use super::{ApiResult, Application};
use crate::{
    models::{item::Item as ItemModel, tag::ItemTag},
    schema,
};

#[derive(schemars::JsonSchema, serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct Item {
    /// Id of the item
    id: i64,
    /// Name of the item
    name: String,
    /// Optional inspection period
    inspection_period_days: Option<i32>,
    /// Optional serial number
    serial_number: Option<String>,
    /// Ids of all tags associated to this item
    tags: Vec<i64>,
}

impl From<(ItemModel, Vec<ItemTag>)> for Item {
    fn from(value: (ItemModel, Vec<ItemTag>)) -> Self {
        Self {
            id: value.0.id,
            name: value.0.name,
            serial_number: value.0.serial_number,
            inspection_period_days: value
                .0
                .inspection_period_days
                .map(|pg_interval| pg_interval.days),
            tags: value
                .1
                .into_iter()
                .map(|item_tag| item_tag.tag_id)
                .collect(),
        }
    }
}

pub async fn handler(state: State<Application>) -> ApiResult<Json<Vec<Item>>> {
    let mut conn = state.database.get().await?;
    let items = schema::items::table
        .get_results::<ItemModel>(&mut conn)
        .await?;
    let tags = ItemTag::belonging_to(&items)
        .get_results::<ItemTag>(&mut conn)
        .await?
        .grouped_by(&items);

    Ok(Json(
        items
            .into_iter()
            .zip(tags)
            .map(|(item_model, item_tags)| (item_model, item_tags).into())
            .collect::<Vec<Item>>(),
    ))
}
