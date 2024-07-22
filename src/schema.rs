// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int8,
        item_id -> Int8,
        parent_id -> Nullable<Int8>,
        ts -> Timestamp,
        data -> Jsonb,
    }
}

diesel::table! {
    items (id) {
        id -> Int8,
        name -> Varchar,
        inspection_period_days -> Nullable<Interval>,
        serial_number -> Nullable<Varchar>,
    }
}

diesel::table! {
    items_tags (id) {
        id -> Int8,
        item_id -> Int8,
        tag_id -> Int8,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        name -> Varchar,
        color -> Varchar,
    }
}

diesel::joinable!(events -> items (item_id));
diesel::joinable!(items_tags -> items (item_id));
diesel::joinable!(items_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    items,
    items_tags,
    tags,
);
