-- This file should undo anything in `up.sql`
ALTER TABLE tags ADD COLUMN color VARCHAR NOT NULL DEFAULT '#ffffff';
