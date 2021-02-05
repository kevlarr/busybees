alter table post add column deleted bool default false;

update post set deleted = false;

alter table post alter column deleted set not null;

drop view post_published_by_date_vw;
drop view post_detail_vw;

create view post_detail_vw as
    select
        author.name as author,
        post.key,
        post.title,
        post.content,
        post.published,
        post.published_at,
        post.deleted,
        preview_image.filename as preview_image_filename,
        preview_image.alt_text as preview_image_alt_text
    from post
    left join author on author.id = post.author_id
    left join post_preview_image_vw preview_image on preview_image.post_id = post.id;

create view post_published_by_date_vw as
    select
        author,
        key,
        title,
        published_at,
        preview_image_filename,
        preview_image_alt_text
    from post_detail_vw
    where published and not deleted
    order by published_at desc;
