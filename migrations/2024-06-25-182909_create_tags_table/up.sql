-- Your SQL goes here
CREATE TABLE tags (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	name VARCHAR NOT NULL, -- name of the category
	color VARCHAR NOT NULL
);

CREATE TABLE items_tags (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    item_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL,
    FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(item_id, tag_id)
);
