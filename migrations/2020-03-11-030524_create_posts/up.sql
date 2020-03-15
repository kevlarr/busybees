-- Your SQL goes here
create table posts (
    id serial primary key,
    alphanumeric_id text unique not null check (length(alphanumeric_id) = 12),
    title text not null check (length(title) <= 128),
    content text not null,
    published boolean default false,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    check(created_at <= updated_at)
);
