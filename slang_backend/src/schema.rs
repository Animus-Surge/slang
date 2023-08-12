// @generated automatically by Diesel CLI.

diesel::table! {
    slang_channels (channel_id) {
        channel_id -> Int4,
        channel_name -> Text,
        channel_msgs -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    slang_groups (group_id) {
        group_id -> Int4,
        group_name -> Text,
        group_roles -> Array<Nullable<Int4>>,
        group_admins -> Array<Nullable<Int4>>,
        group_banlist -> Array<Nullable<Int4>>,
        group_public -> Bool,
    }
}

diesel::table! {
    slang_msgs (message_id) {
        message_id -> Int4,
        message_author -> Int4,
        message_sent -> Text,
        message_edited -> Bool,
        message_content -> Text,
        message_content_type -> Text,
    }
}

diesel::table! {
    slang_roles (role_id) {
        role_id -> Int4,
        role_name -> Text,
        role_color -> Int4,
        role_perms -> Int4,
        role_members -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    slang_users (user_id) {
        user_id -> Int4,
        user_authuid -> Text,
        user_username -> Text,
        user_displayname -> Nullable<Text>,
        user_friends -> Array<Nullable<Int4>>,
        user_blocked -> Array<Nullable<Int4>>,
        user_groups -> Array<Nullable<Int4>>,
        user_flags -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    slang_channels,
    slang_groups,
    slang_msgs,
    slang_roles,
    slang_users,
);
