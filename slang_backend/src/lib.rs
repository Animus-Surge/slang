pub mod schema;
pub mod models;
pub mod messenger;
pub mod dbmanager;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use models::*;
use rocket::Responder;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::env;

use crate::models::Channel;
use crate::schema::slang_groups;

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

// Helper function to check error stuff
fn check_error(option_err: Option<Error>) -> Status {
	match option_err {
		Some(error) => {
			if error == Error::NotFound {return Status::NotFound}
			return Status::InternalServerError
		},
		None => return Status::InternalServerError
	}
}

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

pub fn create_group(group_data: GroupCreate) -> Status {
	Status::NotImplemented
}


// Message handling

pub fn get_message(msgid: i32) -> Result<Vec<Message>, Status> {
	//TODO: group and channel ID handling
	use self::schema::slang_msgs::dsl::*;

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

	let result = slang_msgs
		.filter(message_id.eq(msgid))
		.limit(1)
		.select(Message::as_select())
		.load(conn)
		.expect("Error");

	Ok(result)
}

pub fn create_message(groupid: i32, channelid: i32 ,message: MessageCreate) -> Status {
	use self::schema::slang_msgs;

	use self::schema::slang_groups::dsl::*;
	use self::schema::slang_channels::dsl::*;

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

	//Start with checking for the group
	let group_query: QueryResult<Vec<Group>> = slang_groups.filter(group_id.eq(groupid)).limit(1).select(Group::as_select()).load(conn);
	if group_query.is_err() {
		return check_error(group_query.err());
	}
	let group_res = group_query.ok();

	//Now check for the channel
	let channel_query: QueryResult<Vec<Channel>> = slang_channels.filter(channel_id.eq(channelid)).limit(1).select(Channel::as_select()).load(conn);
	if channel_query.is_err() {
		return check_error(channel_query.err());
	}
	let channel_res = channel_query.ok();

	//For create_message, let's check if the channel is in the group.
	
	//Since group_res and channel_res are options, let's unwrap them
	let gbind = group_res.unwrap();
	let group = gbind.get(0).unwrap();
	let cbind = channel_res.unwrap();
 	let channel = cbind.get(0).unwrap();

	//Now check if channel is in group's channels
	if !group.group_channels.contains(&Some(channel.channel_id)) {
		return Status::BadRequest;
	}

	//Great! We now know that the channel is in the group, so now all we need to do is create the message
	let res = diesel::insert_into(slang_msgs::table)
		.values(&message)
		.returning(Message::as_returning())
		.get_result(conn);
	if res.is_err() {
		return check_error(res.err());
	}

	//Now we need to add it to the channel...
	let ins_value = res.ok().unwrap();
	let channel_add_query = diesel::sql_query(
			format!(
				"UPDATE slang_channels SET channel_msgs = ARRAY_APPEND(channel_msgs, {}) WHERE channel_id = {}", ins_value.message_id, channel.channel_id
			)
		)
		.execute(conn);
	if channel_add_query.is_err() {
		return check_error(channel_add_query.err());
	}

	//And tell the client that the message was created. Done!
	Status::Created
}

// Channel handling

pub fn get_channel(groupid: i32, channelid: i32) -> Status {
	Status::NotImplemented
}

pub fn create_channel(groupid: i32, channel: ChannelCreate) -> Status {
	Status::NotImplemented
}

// User handling

pub fn get_user_exists(userid: i32) -> bool {
	

	false
}