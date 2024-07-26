-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    login VARCHAR NOT NULL,
    -- PHC hash of the password, uses Argon2id v19
    password VARCHAR NOT NULL,
    -- If the user is enabled or disabled
    is_active BOOLEAN NOT NULL,
    -- If the user can manage other users (create users, add permissions)
    perm_users BOOLEAN NOT NULL,
    -- If the user can create/update/delete tags
    perm_tags BOOLEAN NOT NULL,
    -- If the user is authorized create/update/delete items
    perm_items BOOLEAN NOT NULL,
    -- If the user is authorized to do an item inspection
    perm_action_inspect BOOLEAN NOT NULL,
    -- If the use is authorized to lend an item
    perm_action_lend BOOLEAN NOT NULL
);

INSERT INTO users (
    login,
    password,
    is_active,
    perm_users,
    perm_tags,
    perm_items,
    perm_action_inspect,
    perm_action_lend
) VALUES (
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$yyD8iM0p3ou5TgAD6THC+g$/G36laxtzQjtl9ODNfLskuCEEUsy6Hn5H71NmqlVxQ8',
    true,
    true,
    true,
    true,
    true,
    true
);
