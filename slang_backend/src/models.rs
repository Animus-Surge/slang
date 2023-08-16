use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};


/*
TODO BOARD

json handling of objects
*/

// Group Structs

#[derive(Queryable, Selectable, Clone, Serialize)]
#[diesel(table_name = crate::schema::slang_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub group_id: i32,
    pub group_name: String,
    pub group_public: bool,
    pub group_channels: Vec<Option<i32>>,
    pub group_admins: Vec<Option<i32>>,
    pub group_banlist: Vec<Option<i32>>,
    pub group_roles: Vec<Option<i32>>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::slang_groups)]
pub struct GroupCreate<'a> {
    group_name: &'a str
}

//Message Structs

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::slang_msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub message_id: i32,
    pub message_author: i32,
    pub message_sent: String, //Why not NativeDateTime? Because I can't figure out how to implement the Serialize trait for it
    pub message_edited: bool,
    pub message_content: String,
    pub message_content_type: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::slang_msgs)]
#[serde(crate = "rocket::serde")]
pub struct MessageCreate<'a> {
    message_author: i32,
    message_sent: &'a str,
    message_content: &'a str
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::slang_channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Channel {
    pub channel_id: i32,
    pub channel_name: String,
    pub channel_msgs: Vec<Option<i32>>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::slang_channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct ChannelCreate<'a> {
    channel_name: &'a str, 
    channel_msgs: Vec<Option<i32>>
}