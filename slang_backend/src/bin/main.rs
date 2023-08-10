// use diesel::prelude::*;
use slang_backend::*;

#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, http::Status};

/*
BACKEND TODO BOARD

- Messages
- Users
- Groups
...
*/

#[get("/")]
fn index() -> Status {
    Status::Ok
}

//Group queries

// GET: returns information about group [groupid]
#[get("/<groupid>")]
fn group_index(groupid: i32) -> Status {
    let group = get_group_info(groupid);

    Status::NotImplemented
}

// GET: returns the invite link for group [groupid]
#[get("/<groupid>/invite")]
fn group_invite(groupid: i32) -> Status {
    Status::NotImplemented
}

// POST: adds a role to the group
#[get("/<groupid>/addrole")]
fn group_addrole(groupid: i32) -> Status {
    Status::NotImplemented
}

// DELETE: removes a role from the group
#[delete("/<groupid>/removerole/<roleid>")]
fn group_removerole(groupid: i32, roleid: i32) -> Status {
    Status::NotImplemented
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        //mount index roots
        .mount("/", routes![index])
        //mount group routes
        .mount("/groups", routes![group_index])
        //mount user routes
}
