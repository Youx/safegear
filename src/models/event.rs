use chrono::Utc;
use diesel::{
    expression::AsExpression, pg::Pg, sql_types::Jsonb, Associations, ExpressionMethods as _,
    Identifiable, Insertable, OptionalExtension as _, QueryDsl as _, Queryable, Selectable,
};
use diesel_async::{scoped_futures::ScopedFutureExt as _, AsyncConnection as _, RunQueryDsl as _};
use serde::{Deserialize, Serialize};

use crate::{api::ApiError, models::item::Item, schema::*};

#[derive(Insertable)]
#[diesel(table_name = events)]
struct InsertEvent {
    item_id: i64,
    ts: chrono::NaiveDateTime,
    data: EventData,
}

#[derive(Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Item))]
pub struct Event {
    pub id: i64,
    item_id: i64,
    pub ts: chrono::NaiveDateTime,
    pub data: EventData,
}

impl Event {
    pub async fn insert_event(
        conn: &mut diesel_async::AsyncPgConnection,
        item_id: i64,
        ts: chrono::DateTime<Utc>,
        data: EventData,
    ) -> Result<Event, ApiError> {
        conn.transaction(|conn| {
            async {
                let last_event: Option<Self> = events::table
                    .filter(events::item_id.eq(item_id))
                    .order_by(events::ts.desc())
                    .limit(1)
                    .get_result(conn)
                    .await
                    .optional()?;

                // cannot insert an event before another
                if let Some(ref last_event) = last_event {
                    if ts <= last_event.ts.and_utc() {
                        return Err(ApiError::InvalidEventTime(last_event.ts.and_utc(), ts));
                    }
                }

                // only allow specific event successions
                // (ex: can only lose or return an item, after a borrow)
                if !EventData::check_transition(
                    match &last_event {
                        Some(ref value) => Some(&value.data),
                        None => None,
                    },
                    &data,
                ) {
                    Err(ApiError::InvalidTransition(
                        last_event.map(|event| event.data),
                        data,
                    ))
                } else {
                    Ok(InsertEvent {
                        item_id,
                        ts: ts.naive_utc(),
                        data,
                    }
                    .insert_into(events::table)
                    .returning(events::all_columns)
                    .get_result(conn)
                    .await?)
                }
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
#[serde(tag = "kind")]
pub enum EventData {
    /// Event indicating when the item was produced
    Manufactured {} = 0,
    /// Event when someone puts the item in service
    PutIntoService {} = 1,
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

#[derive(Default)]
pub(crate) struct Transition {
    manufactured: bool,
    put_into_service: bool,
    inspected: bool,
    borrowed: bool,
    returned: bool,
    retired: bool,
    lost: bool,
}
impl Transition {
    fn get_value(&self, event: &EventData) -> bool {
        match event {
            EventData::Manufactured {} => self.manufactured,
            EventData::PutIntoService {} => self.put_into_service,
            EventData::Inspected { .. } => self.inspected,
            EventData::Borrowed { .. } => self.borrowed,
            EventData::Returned { .. } => self.returned,
            EventData::Retired {} => self.retired,
            EventData::Lost {} => self.lost,
        }
    }
}

impl EventData {
    fn get_transition(last_event: Option<&Self>) -> Transition {
        match last_event {
            None => Transition {
                manufactured: true,
                put_into_service: false,
                inspected: false,
                borrowed: false,
                returned: false,
                retired: false,
                lost: false,
            },
            Some(EventData::Manufactured {}) => Transition {
                manufactured: false,
                put_into_service: true,
                inspected: false,
                borrowed: false,
                returned: false,
                retired: true,
                lost: true,
            },
            Some(EventData::PutIntoService {}) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: true,
                borrowed: true,
                returned: false,
                retired: true,
                lost: true,
            },
            Some(EventData::Inspected { .. }) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: true,
                borrowed: true,
                returned: false,
                retired: true,
                lost: true,
            },
            Some(EventData::Borrowed { .. }) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: false,
                borrowed: false,
                returned: true,
                retired: false,
                lost: true,
            },
            Some(EventData::Returned { .. }) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: true,
                borrowed: true,
                returned: false,
                retired: true,
                lost: true,
            },
            Some(EventData::Retired { .. }) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: false,
                borrowed: false,
                returned: false,
                retired: false,
                lost: false,
            },
            Some(EventData::Lost { .. }) => Transition {
                manufactured: false,
                put_into_service: false,
                inspected: false,
                borrowed: false,
                returned: false,
                retired: false,
                lost: false,
            },
        }
    }
    pub(crate) fn check_transition(last_event: Option<&Self>, next_event: &Self) -> bool {
        Self::get_transition(last_event).get_value(next_event)
    }
}
