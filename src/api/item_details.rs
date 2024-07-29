use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use diesel::{BelongingToDsl as _, ExpressionMethods as _, QueryDsl as _};
use diesel_async::RunQueryDsl as _;

use super::{ApiResult, Application, AuthenticatedUser, NoPermission};
use crate::{
    models::{event::Event, item::Item as ItemModel, tag::ItemTag},
    schema::*,
};

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct ItemDetails {
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
    /// Events for this item
    events: Vec<ItemEvent>,
}

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct ItemEvent {
    /// Id of the event
    id: i64,
    /// Timestamp of the event
    ts: chrono::DateTime<Utc>,
}

impl From<Event> for ItemEvent {
    fn from(value: Event) -> Self {
        Self {
            id: value.id,
            ts: chrono::DateTime::from_naive_utc_and_offset(value.ts, Utc),
        }
    }
}

impl From<(ItemModel, Vec<ItemTag>, Vec<Event>)> for ItemDetails {
    fn from(value: (ItemModel, Vec<ItemTag>, Vec<Event>)) -> Self {
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
            events: value.2.into_iter().map(|event| event.into()).collect(),
        }
    }
}

pub async fn handler(
    _auth: AuthenticatedUser<NoPermission>,
    state: State<Application>,
    Path(item_id): Path<i64>,
) -> ApiResult<Json<ItemDetails>> {
    let mut conn = state.database.get().await?;
    let item = items::table
        .find(item_id)
        .get_result::<ItemModel>(&mut conn)
        .await?;
    let tags = ItemTag::belonging_to(&item)
        .inner_join(tags::table)
        .order_by(tags::name.asc())
        .select(items_tags::all_columns)
        .get_results::<ItemTag>(&mut conn)
        .await?;
    let events = Event::belonging_to(&item)
        .get_results::<Event>(&mut conn)
        .await?;

    Ok(Json((item, tags, events).into()))
}
