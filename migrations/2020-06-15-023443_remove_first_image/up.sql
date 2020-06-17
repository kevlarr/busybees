create index post_image_image_id_key on post_image (image_id);

create view post_thumbnail_vw as
    select
        post_id,
        coalesce(thumbnail_filename, filename) as filename,
        alt_text
    from post_image
    join image on image.id = post_image.image_id
    where post_image.thumbnail;

drop view published_post_preview_vw;

create view published_post_preview_vw as
    select
        author.name as author,
        post.key,
        post.title,
        post.created_at,
        thumbnail.filename as thumbnail,
        thumbnail.alt_text as thumbnail_alt_text
    from post
    left join author on author.id = post.author_id
    left join post_thumbnail_vw thumbnail on thumbnail.post_id = post.id
    where post.published
    order by created_at desc;

drop view post_detail_vw;

create view post_detail_vw as
    select
        author.name as author,
        post.key,
        post.title,
        post.content,
        post.published,
        post.created_at,
        post.updated_at,
        thumbnail.filename as thumbnail,
        thumbnail.alt_text as thumbnail_alt_text
    from post
    left join author on author.id = post.author_id
    left join post_thumbnail_vw thumbnail on thumbnail.post_id = post.id;

drop function first_image(text);
