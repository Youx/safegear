-- Your SQL goes here
CREATE TABLE items (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	name VARCHAR NOT NULL, -- textual name of the item
	inspection_period_days INTERVAL DAY -- period between inspections for this item
);
