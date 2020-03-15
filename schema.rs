table! {
    posts (id) {
        id -> Int4,
        alphanumeric_id -> Text,
        title -> Text,
        content -> Text,
        published -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
