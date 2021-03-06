use chrono::{DateTime, NaiveTime, Utc};
use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool, Transaction};
use crate::store::{posts::models::*, StoreResult};

pub async fn create(pool: &PgPool, params: NewPostParams) -> StoreResult<String> {
    sqlx::query!(
        "
        insert into post (
                author_id,
                title,
                content,
                published,
                created_at,
                updated_at
        )
            values ($1, $2, $3, false, now(), now())

        returning key
        ",
        params.author_id,
        params.title,
        params.content,
    ).fetch_one(pool).await.map(|row| row.key)
}

pub async fn recent_published(pool: &PgPool) -> StoreResult<Vec<PublishedPostMeta>> {
    sqlx::query_as!(
        PublishedPostMeta,
        "
        select
            author,
            key,
            title,
            published_at,
            preview_image_filename,
            preview_image_alt_text
        from post_published_by_date_vw
        where published_at <= now() -- view includes all `published`
        limit 4
        ",
    ).fetch_all(pool).await
}

pub async fn admin_list(pool: &PgPool) -> StoreResult<Vec<AdminPostMeta>> {
    sqlx::query_as!(
        AdminPostMeta,
        "
        select
            key,
            title,
            published,
            created_at,
            updated_at
        from post
        where not deleted
        order by created_at desc
        ",
    ).fetch_all(pool).await
}

pub async fn get(pool: &PgPool, key: String) -> StoreResult<PostDetail> {
    sqlx::query_as!(
        PostDetail,
        "
        select
            author,
            key,
            title,
            content,
            published,
            published_at,
            preview_image_filename,
            preview_image_alt_text
        from post_detail_vw
        where key = $1
        ",
        key,
    ).fetch_one(pool).await
}

pub async fn update_post(
    tx: &mut Transaction<PoolConnection<PgConnection>>,
    key: String,
    params: UpdatePostParams,
) -> StoreResult<()> {
    let UpdatePostParams { post, linked_uploads, preview_image_id } = params;
    let PostParams { title, content, published_at_date } = post;

    // Hacky, but times aren't displayed anywhere for now, so just
    // default every publish date to midday EST.
    // 17 UTC -> 12-13 EST depending on DST
    let t = NaiveTime::from_hms(17, 0, 0);
    let published_at: Option<DateTime<Utc>> = published_at_date
        .map(|dt| dt.and_time(t))
        .map(|dt| DateTime::from_utc(dt, Utc));

    let post = sqlx::query!(
        "
        update post
        set
            title = $2,
            content = $3,
            published_at = $4,
            updated_at = now()
        where key = $1
        returning id
        ",
        key,
        title,
        content,
        published_at,
    ).fetch_one(&mut *tx).await?;

    sqlx::query!(
        "
        delete from post_image
        using image
        where
            image.id = image_id and
            post_id = $1 and
            image.filename <> all($2)
        ",
        post.id,
        &linked_uploads
    ).execute(&mut *tx).await?;

    // This is less than ideal, but until support for dynamic VALUES list drops,
    // I would rather take the performance hit of multiple queries in favor
    // of having compile-time guarantees.
    //
    // See: https://github.com/launchbadge/sqlx/issues/291
    for filename in linked_uploads {
        sqlx::query!(
            "
            insert into post_image (post_id, image_id)
                values ($1, (select id from image where filename = $2))

                -- Conflict could arise not only from trying to save a linked upload
                -- that already exists, but also from a post that links to the same
                -- file twice. The latter probably isn't common, but it shouldn't lead
                -- to a duplicate record.
                on conflict do nothing
            ",
            post.id,
            filename,
        ).execute(&mut *tx).await?;
    }

    if let Some(image_id) = preview_image_id {
        // This two-updates/one-statement approach actually helps to minimize
        // *both* updating rows needlessly *and* number of statements issued.
        // Eg. if the same post-image is selected, 0 updates will be made.
        sqlx::query!(
            "
            with cleared as (
                update post_image
                set is_preview = false
                where post_id = $1 and image_id != $2 and is_preview
                returning id
            )
            update post_image a
            set is_preview = true

            -- Results from CTE might be empty, but even if not this needs an outer join
            -- to re-select the post-image with image_id = $2 for that row to be updated.
            from cleared
            right join post_image b on
                cleared.id = b.id and
                b.post_id = $1

            where a.image_id = $2;
            ",
            post.id,
            image_id,
        ).execute(&mut *tx).await?;
    }

    Ok(())
}

pub async fn update_status(pool: &PgPool, key: String, published: bool) -> StoreResult<()> {
    sqlx::query!(
        "
        update post set published = $2, updated_at = now() where key = $1
        ",
        key,
        published,
    ).execute(pool).await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, key: &str) -> StoreResult<()> {
    sqlx::query!(
        "
        update post set deleted = true where key = $1
        ",
        key.to_string(),
    ).execute(pool).await?;

    Ok(())
}
