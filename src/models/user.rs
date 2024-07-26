use diesel::prelude::*;

use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub password: String,
    pub is_active: bool,
    pub perm_users: bool,
    pub perm_tags: bool,
    pub perm_items: bool,
    pub perm_action_inspect: bool,
    pub perm_action_lend: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct InsertUser {
    id: i64,
    login: String,
    password: String,
    is_active: bool,
    perm_users: bool,
    perm_tags: bool,
    perm_items: bool,
    perm_action_inspect: bool,
    perm_action_lend: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct UpdateUserAdmin {
    login: Option<String>,
    is_active: Option<bool>,
    perm_users: Option<bool>,
    perm_tags: Option<bool>,
    perm_items: Option<bool>,
    perm_action_inspect: Option<bool>,
    perm_action_lend: Option<bool>,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct UpdateUserSelf {
    password: Option<String>,
}
