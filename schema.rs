table! {
    author (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        name -> Text,
    }
}

table! {
    image (id) {
        id -> Int4,
        src -> Text,
        alt -> Nullable<Text>,
    }
}

table! {
    post (id) {
        id -> Int4,
        key -> Text,
        title -> Text,
        content -> Nullable<Text>,
        published -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        author_id -> Nullable<Int4>,
    }
}

table! {
    post_image (id) {
        id -> Int4,
        post_id -> Int4,
        image_id -> Int4,
        thumbnail -> Nullable<Bool>,
    }
}

joinable!(post -> author (author_id));

allow_tables_to_appear_in_same_query!(
    author,
    image,
    post,
    post_image,
);
