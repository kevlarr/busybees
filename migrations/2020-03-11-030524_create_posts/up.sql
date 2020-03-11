-- Your SQL goes here
create table posts (
    id serial primary key,
    title varchar not null,
    body text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    check(created_at <= updated_at)
);
