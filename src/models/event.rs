use diesel::{
    expression::AsExpression, pg::Pg, sql_types::Jsonb, Associations, ExpressionMethods as _,
    Identifiable, Insertable, QueryDsl as _, Queryable, Selectable,
};
use diesel_async::{scoped_futures::ScopedFutureExt as _, AsyncConnection as _, RunQueryDsl as _};
use serde::{Deserialize, Serialize};

use crate::{models::item::Item, schema::*};

#[derive(Insertable)]
#[diesel(table_name = events)]
struct InsertEvent {
    item_id: i64,
    parent_id: Option<i64>,
    ts: chrono::NaiveDateTime,
    data: EventData,
}

#[derive(Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(Event, foreign_key = parent_id))]
pub struct Event {
    pub id: i64,
    item_id: i64,
    parent_id: Option<i64>,
    pub ts: chrono::NaiveDateTime,
    pub data: EventData,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parent event is missing")]
    ParentEventMissing,
    #[error(transparent)]
    Database(#[from] diesel::result::Error),
    #[error("Invalid parent kind")]
    InvalidParentKind,
    #[error("Event already exists for this item")]
    Duplicate,
}

impl Event {
    pub async fn insert_event(
        conn: &mut diesel_async::AsyncPgConnection,
        item_id: i64,
        ts: chrono::NaiveDateTime,
        data: EventData,
    ) -> Result<Event, Error> {
        if data.parent_discriminant().is_some() {
            return Err(Error::ParentEventMissing);
        }
        conn.transaction(|conn| {
            async {
                if data.is_unique() {
                    let events: Vec<Self> = events::table
                        .filter(events::item_id.eq(item_id))
                        .get_results(conn)
                        .await?;
                    if events
                        .iter()
                        .any(|event| event.data.discriminant() == data.discriminant())
                    {
                        return Err(Error::Duplicate);
                    }
                }
                Ok(InsertEvent {
                    item_id,
                    parent_id: None,
                    ts,
                    data,
                }
                .insert_into(events::table)
                .returning(events::all_columns)
                .get_result(conn)
                .await?)
            }
            .scope_boxed()
        })
        .await
    }

    pub async fn insert_sub_event(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_id: i64,
        ts: chrono::NaiveDateTime,
        data: EventData,
    ) -> Result<Event, Error> {
        conn.transaction(|conn| {
            async {
                let parent_event: Event = events::table.find(parent_id).get_result(conn).await?;
                if Some(parent_event.data.discriminant()) != data.parent_discriminant() {
                    return Err(Error::InvalidParentKind);
                }
                if data.is_unique() {
                    let events: Vec<Self> = events::table
                        .filter(events::item_id.eq(parent_event.item_id))
                        .get_results(conn)
                        .await?;
                    if events
                        .iter()
                        .any(|event| event.data.discriminant() == data.discriminant())
                    {
                        return Err(Error::Duplicate);
                    }
                }

                Ok(InsertEvent {
                    item_id: parent_event.item_id,
                    parent_id: Some(parent_id),
                    ts,
                    data,
                }
                .insert_into(events::table)
                .returning(events::all_columns)
                .get_result(conn)
                .await?)
            }
            .scope_boxed()
        })
        .await
    }
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
pub enum InspectionResult {
    /// Item is new or in very good condition
    Good,
    /// Item shows signs of normal use
    NormalWear,
    /// Item seems close to end of life
    Warning,
    /// Item must be retired
    Danger,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug, AsExpression)]
#[diesel(sql_type = Jsonb)]
#[repr(u8)]
pub enum EventData {
    /// Event indicating when the item was produced
    Produced {} = 0,
    /// Event when someone puts the item in service
    PutInService {} = 1,
    /// Event when someone inspects the item
    Inspected {
        /// Name of the person who inspected the item
        inspector: String,
        /// Result of the
        result: InspectionResult,
        /// Optional comment of the inspector
        comment: Option<String>,
    } = 2,
    /// Event logged when someone borrows an item
    Borrowed {
        /// Person who borrowed the item
        borrower: String,
        /// Person who validated the borrow
        validator: String,
    } = 3,
    /// Event logged when an item is returned after a borrow
    Returned {
        /// Person who validated the borrow
        validator: String,
    } = 4,
    /// Event logged when the item is retired
    Retired {} = 5,
    /// Event logged when the item is retired
    Lost {} = 6,
}
diesel_json!(EventData);

impl EventData {
    /// Indicates if this event can happen only once.
    pub(crate) fn is_unique(&self) -> bool {
        match self {
            EventData::Produced {} => true,
            EventData::PutInService {} => true,
            EventData::Inspected { .. } => false,
            EventData::Borrowed { .. } => false,
            EventData::Returned { .. } => false,
            EventData::Retired {} => true,
            EventData::Lost {} => true,
        }
    }
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
    /// Indicates the kind of the parent event.
    fn parent_discriminant(&self) -> Option<u8> {
        match self {
            EventData::Produced {} => None,
            EventData::PutInService {} => None,
            EventData::Inspected { .. } => None,
            EventData::Borrowed { .. } => None,
            EventData::Returned { .. } => Some(
                EventData::Borrowed {
                    borrower: "".to_owned(),
                    validator: "".to_owned(),
                }
                .discriminant(),
            ),
            EventData::Retired {} => Some(EventData::PutInService {}.discriminant()),
            EventData::Lost {} => Some(EventData::PutInService {}.discriminant()),
        }
    }
}
