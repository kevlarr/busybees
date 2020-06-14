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
