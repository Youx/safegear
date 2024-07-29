-- Your SQL goes here
ALTER TABLE users
ADD CONSTRAINT user_login_unique
UNIQUE (login);
