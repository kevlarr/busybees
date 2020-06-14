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
