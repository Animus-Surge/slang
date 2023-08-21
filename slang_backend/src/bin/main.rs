use slang_backend::{*, models::*};

#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, http::Status};

// Utility members

fn generate_response<T>(query_handle: Result<Vec<T>, Status>) -> SlangResponse<Vec<T>> {
    if query_handle.is_err() {
        match query_handle.err() {
            Some(status) => return SlangResponse::ErrorResponse(status.to_string()),
            None => return SlangResponse::ErrorResponse("Some Error".to_string())
        }
    }

    let group = query_handle.unwrap();

    if group.len() == 0 {
        return SlangResponse::NotFoundResponse("Data not found".to_string());
    }

    SlangResponse::QueryResponse(Json(group))
}

//Root index
#[get("/")]
fn index() -> Status {
    Status::Ok
}

//Group queries

// POST: creates a new group
#[post("/create")]
fn group_create() -> SlangResponse<String> {
    SlangResponse::NoImplResponse("No group create functionality yet".to_string())
}

// GET: returns information about group [groupid]
#[get("/<groupid>")]
fn group_index(groupid: i32) -> SlangResponse<Vec<Group>> {
    generate_response(get_group_info(groupid))
}

// GET: returns the invite link for group [groupid]
#[get("/<groupid>/invite")]
fn group_invite(groupid: i32) -> Status {
    Status::NotImplemented
}

// POST: adds a role to the group
#[post("/<groupid>/addrole")]
fn group_addrole(groupid: i32) -> Status {
    Status::NotImplemented
}

#[get("/<groupid>/roles/<roleid>")]
fn group_getrole(groupid: i32, roleid: i32) -> Status {
    Status::NotImplemented
}

#[patch("/<groupid>/roles/<roleid>/update")]
fn group_editrole(groupid: i32, roleid: i32) -> Status {
    Status::NotImplemented 
}

#[delete("/<groupid>/roles/<roleid>/delete")]
fn group_deleterole(groupid: i32, roleid: i32) -> Status {
    Status::NotImplemented
}

#[post("/<groupid>/create", data = "<channel>")]
fn group_createchannel(groupid: i32, channel: Json<ChannelCreate>) -> Status {
    create_channel(groupid, channel.0)
}

#[get("/<groupid>/<channelid>")]
fn group_getchannel(groupid: i32, channelid: i32) -> Status {
    get_channel(groupid, channelid)
}

#[post("/<groupid>/<channelid>/send", data = "<message>")]
fn channel_sendmsg(groupid: i32, channelid: i32, message: Json<MessageCreate>) -> Status {
    //TODO channel and group handling

    create_message(groupid, channelid, message.0)
}

#[get("/<groupid>/<channelid>/<messageid>")]
fn channel_getmessage(groupid: i32, channelid: i32, messageid: i32) -> SlangResponse<Vec<Message>> {
    generate_response(get_message(messageid))
}

#[get("/<userid>/exists")]
fn user_checkexists(userid: i32) -> bool {
    

    false
}

#[launch]
fn rocket() -> _ {
    //Start the messenger socket listeners
    

    //Launch Rocket
    rocket::build()
        //mount index roots
        .mount("/", routes![index]) 
        //mount group routes
        .mount("/groups", routes![group_index, group_create, channel_sendmsg, channel_getmessage])
        //mount user routes
}
