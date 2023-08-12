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
    group_id: i32,
    group_name: String,
    group_public: bool,
    group_admins: Vec<Option<i32>>,
    group_banlist: Vec<Option<i32>>,
    group_roles: Vec<Option<i32>>
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
    message_id: i32,
    message_author: i32,
    message_sent: String, //Why not NativeDateTime? Because I can't figure out how to implement the Serialize trait for it
    message_edited: bool,
    message_content: String,
    message_content_type: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::slang_msgs)]
#[serde(crate = "rocket::serde")]
pub struct MessageCreate<'a> {
    message_author: i32,
    message_sent: &'a str,
    message_content: &'a str
}