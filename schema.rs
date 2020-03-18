table! {
    post (id) {
        id -> Int4,
        key -> Text,
        title -> Text,
        content -> Text,
        published -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
