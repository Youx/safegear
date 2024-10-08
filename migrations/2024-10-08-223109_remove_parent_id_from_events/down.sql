-- This file should undo anything in `up.sql`
ALTER TABLE events
ADD COLUMN parent_id BIGINT;

ALTER TABLE events
ADD CONSTRAINT events_parent_id_fkey FOREIGN KEY(parent_id)
    REFERENCES events(id)
    ON DELETE CASCADE
