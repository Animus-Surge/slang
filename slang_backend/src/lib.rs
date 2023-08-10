pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Group;
use rocket::http::Status;
use std::env;

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

pub fn get_group_info(gid: i32) -> Group {
	use self::schema::slang_groups::dsl::*;

	let conn = &mut establish_connection().unwrap();

	let result = slang_groups
		.filter(group_id.eq(gid))
		.limit(1)
		.select(Group::as_select())
		.load(conn)
		.expect("Error");

	result[0].clone()
}

