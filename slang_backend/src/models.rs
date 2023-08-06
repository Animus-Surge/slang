use diesel::prelude::*;

use rocket::serde::{Serialize, Deserialize};

use crate::schema::messages;


#[derive(Queryable, Selectable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
	pub message_id: i32,
	pub message_author: i32,
	pub message_text: String,
	pub message_sent: String
}

#[derive(Serialize, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = messages)]
pub struct _Message<'a> {
	pub message_author: i32,
	pub message_text: &'a str,
	pub message_sent: &'a str
}