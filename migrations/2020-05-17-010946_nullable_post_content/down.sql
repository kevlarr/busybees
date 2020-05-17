alter table post
    alter column content set not null,
    add constraint post_content_check check (length(content) > 0);
