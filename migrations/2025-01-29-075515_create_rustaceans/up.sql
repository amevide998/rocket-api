-- Your SQL goes here
create table rustaceans (
    id integer not null primary key autoincrement,
    name varchar not null,
    email varchar not null,
    create_at timestamp not null default current_timestamp
)