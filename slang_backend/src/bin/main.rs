use self::models::_Message;
use diesel::prelude::*;
use slang_backend::*;

#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, http::Status};

/*
BACKEND TODO BOARD

- Groups
...
*/

#[get("/")]
fn index() -> &'static str {
    "Slang root index"
}

#[get("/")]
fn list() -> Status {
    Status::NetworkAuthenticationRequired
}

#[post("/create", data="<message>")]
fn send(message: Json<_Message>) -> Status {
    add_message(message.message_author, message.message_text, message.message_sent)
}

#[get("/<message>")]
fn get(message: i32) -> Status {
    //TODO: write logic to find the stuff

    Status::InternalServerError
}

#[launch]
fn rocket() -> _ {


    rocket::build()
        .mount("/", routes![index])
        .mount("/messages", routes![get, list, send])
}
