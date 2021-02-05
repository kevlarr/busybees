-- Your SQL goes here
alter table post
    add column published_at timestamp with time zone;

-- "created_at" has been the defacto publication time
update post
    set published_at = created_at
    where published;

create index post_published_at_idx
    on post (published_at)
    where published;

drop view post_published_by_date_vw;
drop view post_detail_vw;

create view post_detail_vw as
    SELECT
        author.name AS author,
        post.key,
        post.title,
        post.content,
        post.published,
        post.published_at,
        preview_image.filename AS preview_image_filename,
        preview_image.alt_text AS preview_image_alt_text
    FROM post
    LEFT JOIN author ON author.id = post.author_id
    LEFT JOIN post_preview_image_vw preview_image ON preview_image.post_id = post.id;

create view post_published_by_date_vw as
    SELECT
        author,
        key,
        title,
        published_at,
        preview_image_filename,
        preview_image_alt_text
    FROM post_detail_vw
    WHERE published
    ORDER BY published_at DESC;
