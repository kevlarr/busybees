alter table post
    add constraint post_content_check check (length(content) > 0);
