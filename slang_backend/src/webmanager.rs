/*
    SLANG BACKEND - WEB MANAGER
    This file contains the REST API for the Slang backend.

    Author: Surge
    Version: 1.0
    License: Apache 2.0
*/
use rocket::{serde::json::Json, http::Status};

fn generate_response<T>(query_handle: Result<Vec<T>, Status>) {

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn create_instance() -> _ {
    //Launch the rocket instance
    rocket::build()
        .mount("/", routes![index])
}