-- Your SQL goes here
create table posts (
    id serial primary key,
    alphanumeric_id varchar(16) unique not null,
    title varchar not null,
    content text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    check(created_at <= updated_at)
);
