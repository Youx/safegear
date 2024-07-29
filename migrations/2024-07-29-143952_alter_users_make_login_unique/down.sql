-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP CONSTRAINT user_login_unique;
