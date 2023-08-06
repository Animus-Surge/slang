// @generated automatically by Diesel CLI.

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        message_author -> Int4,
        message_text -> Text,
        message_sent -> Text,
        message_edited -> Bool,
    }
}
