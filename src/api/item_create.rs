use axum::{extract::State, Json};
use chrono::Utc;
use diesel::{data_types::PgInterval, BelongingToDsl};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection as _, RunQueryDsl as _};

use crate::{
    models::{
        event::{Event, EventData},
        item::{InsertItem as InsertItemModel, Item as ItemModel},
        tag::{InsertItemTag, ItemTag},
    },
    schema::*,
};

use super::{item_list::Item, ApiError, ApiResult, Application, AuthenticatedUser, ManageItems};

#[derive(serde::Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct CreateItem {
    name: String,
    inspection_period_days: Option<i32>,
    serial_number: Option<String>,
    tags: Vec<i64>,
    manufactured_on: Option<chrono::DateTime<Utc>>,
    put_into_service_on: Option<chrono::DateTime<Utc>>,
}

pub async fn handler(
    _auth: AuthenticatedUser<ManageItems>,
    state: State<Application>,
    Json(data): Json<CreateItem>,
) -> ApiResult<Json<Item>> {
    let mut conn = state.database.get().await?;
    let CreateItem {
        tags,
        name,
        serial_number,
        inspection_period_days,
        manufactured_on,
        put_into_service_on,
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

                if let Some(manufactured_on) = manufactured_on {
                    Event::insert_event(conn, item.id, manufactured_on, EventData::Manufactured {})
                        .await?;
                }
                if let Some(put_into_service_on) = put_into_service_on {
                    Event::insert_event(
                        conn,
                        item.id,
                        put_into_service_on,
                        EventData::PutIntoService {},
                    )
                    .await?;
                }

                let item_tags = ItemTag::belonging_to(&item)
                    .get_results::<ItemTag>(&mut conn)
                    .await?;

                Ok::<_, ApiError>((item, item_tags))
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json((item, item_tags).into()))
}
