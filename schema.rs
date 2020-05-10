table! {
    author (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        name -> Text,
    }
}

table! {
    page_view (id) {
        id -> Int4,
        viewed_at -> Timestamptz,
        pathname -> Text,
        ip -> Nullable<Text>,
        referer -> Nullable<Text>,
    }
}

table! {
    post (id) {
        id -> Int4,
        key -> Text,
        title -> Text,
        content -> Text,
        published -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        author_id -> Nullable<Int4>,
    }
}

joinable!(post -> author (author_id));

allow_tables_to_appear_in_same_query!(
    author,
    page_view,
    post,
);
