pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
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

