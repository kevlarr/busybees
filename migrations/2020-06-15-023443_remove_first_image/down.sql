create function first_image(content text) returns text as $$
    select substring(content, 'src="([a-zA-Z0-9\.\-_~:\/%\?#=]+)"');
$$ language sql immutable;

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
        first_image(post.content) as thumbnail
    from post
    left join author on author.id = post.author_id;

drop view published_post_preview_vw;

create view published_post_preview_vw as
    select
        author.name as author,
        key,
        title,
        created_at,
        first_image(content) as thumbnail
    from post
    left join author on author.id = post.author_id
    where published
    order by created_at desc;

drop view post_thumbnail_vw;

drop index post_image_image_id_key;
