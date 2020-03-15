table! {
    posts (id) {
        id -> Int4,
        alphanumeric_id -> Varchar,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
