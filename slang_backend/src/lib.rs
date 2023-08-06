pub mod schema;
pub mod models;

use self::models::{Message, _Message};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use schema::messages;
use std::env;

pub fn establish_connection() -> Result<PgConnection, Status> {
	dotenv().ok();

	let dburl = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let conn = PgConnection::establish(&dburl);

	if conn.is_err() {
		return Err(Status::InternalServerError);
	}

	Ok(conn.unwrap())
}

pub fn add_message(author: i32, text: &str, authored: &str) -> Status {
	let conn = establish_connection();

	if conn.is_err() {return Status::InternalServerError;}

	let ms = _Message{message_author: author, message_text: text, message_sent: authored};

	let ins = diesel::insert_into(messages::table)
		.values(&ms)
		.execute(&mut conn.unwrap());

	if ins.is_err() {return Status::InternalServerError;}

	Status::Ok
}