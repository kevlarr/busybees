alter table post
    alter column content drop not null,
    drop constraint post_content_check;
