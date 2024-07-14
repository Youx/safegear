use diesel::{
    expression::AsExpression, pg::Pg, sql_types::Jsonb, Associations, Identifiable, Insertable,
    QueryDsl as _, Queryable, Selectable,
};
use diesel_async::RunQueryDsl as _;
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

impl Event {
    pub async fn insert_event(
        conn: &mut diesel_async::AsyncPgConnection,
        item_id: i64,
        ts: chrono::NaiveDateTime,
        data: EventData,
    ) -> Result<Event, diesel::result::Error> {
        InsertEvent {
            item_id,
            parent_id: None,
            ts,
            data,
        }
        .insert_into(events::table)
        .returning(events::all_columns)
        .get_result(conn)
        .await
    }

    pub async fn insert_sub_event(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_id: i64,
        ts: chrono::NaiveDateTime,
        data: EventData,
    ) -> Result<Event, diesel::result::Error> {
        let item_id = events::table
            .find(parent_id)
            .select(events::item_id)
            .get_result(conn)
            .await?;
        InsertEvent {
            item_id,
            parent_id: Some(parent_id),
            ts,
            data,
        }
        .insert_into(events::table)
        .returning(events::all_columns)
        .get_result(conn)
        .await
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

impl diesel::Queryable<Jsonb, Pg> for EventData {
    type Row = serde_json::Value;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(serde_json::from_value(row)?)
    }
}
impl diesel::deserialize::FromSql<Jsonb, Pg> for EventData {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let v = <serde_json::Value as diesel::deserialize::FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        let event_data = serde_json::from_value(v)?;
        Ok(event_data)
    }
}
impl diesel::serialize::ToSql<Jsonb, Pg> for EventData {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let v = serde_json::to_value(self)?;
        <serde_json::Value as diesel::serialize::ToSql<Jsonb, Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

#[derive(Serialize, Deserialize, Debug, AsExpression)]
#[diesel(sql_type = Jsonb)]
#[repr(u8)]
pub enum EventData {
    /// Event when someone puts the item in service
    PutInService {} = 0,
    /// Event when someone inspects the item
    InspectionEvent {
        /// Name of the person who inspected the item
        inspector: String,
        /// Result of the
        result: InspectionResult,
        /// Optional comment of the inspector
        comment: Option<String>,
    } = 1,
    /// Event logged when someone borrows an item
    BorrowedEvent {
        /// Person who borrowed the item
        borrower: String,
        /// Person who validated the borrow
        validator: String,
    } = 2,
    /// Event logged when an item is returned after a borrow
    ReturnedEvent {
        /// Person who validated the borrow
        validator: String,
    } = 3,
    /// Event logged when the item is retired
    Retire {} = 4,
}

impl EventData {
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
    fn parent_discriminant(&self) -> Option<u8> {
        match self {
            EventData::PutInService {} => None,
            EventData::InspectionEvent { .. } => None,
            EventData::BorrowedEvent { .. } => None,
            EventData::Retire {} => None,
            EventData::ReturnedEvent { .. } => Some(2),
        }
    }
}
