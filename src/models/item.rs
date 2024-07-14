use diesel::{data_types::PgInterval, prelude::*};

use crate::schema::items;

#[derive(Selectable, Identifiable, Queryable)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub inspection_period_days: Option<PgInterval>,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct InsertItem {
    pub name: String,
    pub inspection_period_days: Option<PgInterval>,
}
