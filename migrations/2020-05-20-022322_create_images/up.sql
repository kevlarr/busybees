create table image (
    id serial primary key,
    filename text unique not null,
    thumbnail_filename text unique,
    alt_text text,
    width smallint not null,
    height smallint not null,
    kb int
);

create table post_image (
    id serial primary key,
    post_id int not null references post (id),
    image_id int not null references image (id),
    thumbnail boolean default false,

    unique (post_id, image_id)
);

create unique index post_image_thumbnail_post_key
    on post_image (thumbnail, post_id) where thumbnail;
