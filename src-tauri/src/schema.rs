// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Integer,
        note_id -> Integer,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        last_updated -> Timestamp,
    }
}
