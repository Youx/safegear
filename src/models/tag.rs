use diesel::prelude::*;

use crate::schema::*;

use super::item::Item;

#[derive(Identifiable, Queryable)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct InsertTag {
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(Item))]
#[diesel(table_name = items_tags)]
pub struct ItemTag {
    id: i64,
    pub item_id: i64,
    pub tag_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = items_tags)]
pub struct InsertItemTag {
    pub item_id: i64,
    pub tag_id: i64,
}
