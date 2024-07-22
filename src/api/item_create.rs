use axum::{extract::State, Json};
use diesel::{data_types::PgInterval, BelongingToDsl};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection as _, RunQueryDsl as _};

use crate::{
    models::{
        item::{InsertItem as InsertItemModel, Item as ItemModel},
        tag::{InsertItemTag, ItemTag},
    },
    schema::*,
};

use super::{item_list::Item, ApiResult, Application};

#[derive(serde::Deserialize, schemars::JsonSchema, ts_rs::TS)]
#[ts(export)]
pub struct CreateItem {
    name: String,
    inspection_period_days: Option<i32>,
    serial_number: Option<String>,
    tags: Vec<i64>,
}

pub async fn handler(
    state: State<Application>,
    Json(data): Json<CreateItem>,
) -> ApiResult<Json<Item>> {
    let mut conn = state.database.get().await?;
    let CreateItem {
        tags,
        name,
        serial_number,
        inspection_period_days,
    } = data;
    let (item, item_tags) = conn
        .transaction(|mut conn| {
            async move {
                let item = diesel::insert_into(items::table)
                    .values(InsertItemModel {
                        name,
                        serial_number,
                        inspection_period_days: inspection_period_days.map(PgInterval::from_days),
                    })
                    .returning(items::all_columns)
                    .get_result::<ItemModel>(&mut conn)
                    .await?;

                diesel::insert_into(items_tags::table)
                    .values(
                        tags.into_iter()
                            .map(|tag_id| InsertItemTag {
                                item_id: item.id,
                                tag_id,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .execute(&mut conn)
                    .await?;

                let item_tags = ItemTag::belonging_to(&item)
                    .get_results::<ItemTag>(&mut conn)
                    .await?;
                Ok::<_, diesel::result::Error>((item, item_tags))
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json((item, item_tags).into()))
}
