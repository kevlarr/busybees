alter table post_image
    drop constraint post_image_post_id_fkey,

    add constraint post_image_post_id_fkey
        foreign key (post_id) references post (id);
