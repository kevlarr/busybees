create table image (
    id serial primary key,
    src text unique not null,
    thumbnail_src text unique,
    alt text
);

create table post_image (
    id serial primary key,
    post_id int not null,
    image_id int not null,
    thumbnail boolean default false,

    unique (post_id, image_id)
);

create unique index post_image_post_thubnail_key
    on post_image (post_id, thumbnail) where thumbnail;
