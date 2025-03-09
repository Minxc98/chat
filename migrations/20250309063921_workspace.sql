--CREATE TABLE IF NOT EXISTS users (
--    id SERIAL PRIMARY KEY,
--    username VARCHAR(50) NOT NULL UNIQUE,
--    password VARCHAR(255) NOT NULL,
--    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
--);
--更改 password 为 password_hash
ALTER TABLE users RENAME COLUMN password TO password_hash;
--添加email字段
ALTER TABLE users ADD COLUMN email VARCHAR(255) NOT NULL UNIQUE;
--添加workspace
create table if not exists workspaces (
    id serial primary key,
    name varchar(50) not null,
    owner_id integer not null,
    created_at timestamp default current_timestamp
);
--添加workspace for users
ALTER TABLE users ADD COLUMN ws_id INTEGER Not NULL DEFAULT 1 ;

