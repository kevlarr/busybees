-- view is no longer used
drop view post_detail_vw
;

-- drop to re-create without "first_image" function
drop view published_post_preview_vw
;
drop function first_image(text)
;

-- "thumbnail" was a confusing term for a post's preview image,
-- given that "thumbnail" was also an attribute of every image
alter table post_image rename thumbnail to is_preview
;

-- selects a preview image for each post, looking first for
-- an image marked as `is_preview` and then resorting to the
-- first image uploaded for the post if none are marked
create index post_image_image_id_key on post_image (image_id)
;

create view post_preview_image_vw as
    select distinct on (post_id)
        post_id,
        coalesce(thumbnail_filename, filename) as filename,
        alt_text
    from post_image
    join image on image.id = post_image.image_id
    order by
        post_id asc,
        post_image.is_preview desc,
        post_image.id asc
;

-- view for loading everything needed to display post and
-- support relevant `<meta>` tags (eg. OG), loading preview
-- image from `post_image` rather than using `first_image`
create view post_detail_vw as
    select
        author.name as author,
        post.key,
        post.title,
        post.content,
        post.created_at,
        post.published,
        preview_image.filename as preview_image_filename,
        preview_image.alt_text as preview_image_alt_text
    from post
    left join author on author.id = post.author_id
    left join post_preview_image_vw preview_image on preview_image.post_id = post.id
;

-- re-created view relying on `post_image` table instead of
-- the `first_image` function
create view post_published_by_date_vw as
    select
        author,
        key,
        title,
        created_at,
        preview_image_filename,
        preview_image_alt_text
    from post_detail_vw
    where published
    order by created_at desc
;
