-- Your SQL goes here
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    item_id BIGINT NOT NULL, -- item this event applies to
    parent_id BIGINT, -- if the event refers to another one
    ts TIMESTAMP NOT NULL, -- time the event was registered
    data JSONB NOT NULL, -- json of the event (XXX: TEXT for diesel compat)
    FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_id) REFERENCES events(id) ON DELETE CASCADE
);
