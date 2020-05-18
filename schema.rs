table! {
    author (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        name -> Text,
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
    post,
);
