pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use models::{Group, Message, MessageCreate};
use rocket::Responder;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::env;

// Response types
#[derive(Responder)]
pub enum SlangResponse<T> {
	#[response(status = 200)]
	QueryResponse(Json<T>),
	#[response(status = 201)]
	OkResponse(String),

	#[response(status = 500)]
	ErrorResponse(String),
	#[response(status = 404)]
	NotFoundResponse(String),
	#[response(status = 401)]
	UnauthorizedResponse(String),
	#[response(status = 403)]
	NoAccessResponse(String),
	#[response(status = 501)]
	NoImplResponse(String)
}

//TODO: rewrites and comments

pub fn establish_connection() -> Result<PgConnection, Status> {
	dotenv().ok();

	let dburl = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let conn = PgConnection::establish(&dburl);

	if conn.is_err() {
		return Err(Status::InternalServerError);
	}

	Ok(conn.unwrap())
}

// Group Handling

pub fn get_group_info(gid: i32) -> Result<Vec<Group>, Status> {
	use self::schema::slang_groups::dsl::*;

	//Get the connection to the database
	let est = establish_connection();

	if est.is_err() {
		match est.err() {
			Some(status) => return Err(status),
			// Some(_) => return Err(Status::InternalServerError),
			None => return Err(Status::InternalServerError)
		}
	}
	let conn = &mut establish_connection().unwrap();

	//Now get the group
	let result = slang_groups
		.filter(group_id.eq(gid))
		.limit(1)
		.select(Group::as_select())
		.load(conn)
		.expect("Error");

	Ok(result)
}

pub fn create_group(group_data: String) -> Status {
	Status::NotImplemented
}


// Message handling

pub fn get_message(msgid: i32) -> Result<Vec<Message>, Status> {
	Err(Status::NotImplemented)
}

pub fn create_message(/*TODO: channel and group management*/message: MessageCreate) -> Status {
	use self::schema::slang_msgs;

	//Get the connection to the database
	let est = establish_connection();

	if est.is_err() {
		match est.err() {
			Some(status) => return status,
			// Some(_) => return Err(Status::InternalServerError),
			None => return Status::InternalServerError
		}
	}
	let conn = &mut establish_connection().unwrap();

	let res: Result<usize, diesel::result::Error> = diesel::insert_into(slang_msgs::table)
		.values(&message)
		.execute(conn);

	if res.is_err() {
		let err = res.err();

		match err {
			Some(error) => {
				if error == Error::NotFound {return Status::NotFound}
				return Status::InternalServerError
			},
			None => return Status::InternalServerError
		}
	}

	Status::Created
}