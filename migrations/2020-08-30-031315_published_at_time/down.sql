drop view post_published_by_date_vw;
drop view post_detail_vw ;

create view post_detail_vw as
    SELECT
        author.name AS author,
        post.key,
        post.title,
        post.content,
        post.created_at,
        post.published,
        preview_image.filename AS preview_image_filename,
        preview_image.alt_text AS preview_image_alt_text
    FROM post
    LEFT JOIN author ON author.id = post.author_id
    LEFT JOIN post_preview_image_vw preview_image ON preview_image.post_id = post.id;

create view post_published_by_date_vw as
    SELECT
        post_detail_vw.author,
        post_detail_vw.key,
        post_detail_vw.title,
        post_detail_vw.created_at,
        post_detail_vw.preview_image_filename,
        post_detail_vw.preview_image_alt_text
    FROM post_detail_vw
    WHERE post_detail_vw.published
    ORDER BY post_detail_vw.created_at DESC;


alter table post
    drop column published_at;

