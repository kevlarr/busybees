create table author (
    id serial primary key,
    email text unique not null,
    password_hash text not null,
    name text not null
);

alter table post add column author_id int references author(id);
create index post_author_id_idx on post (author_id);
