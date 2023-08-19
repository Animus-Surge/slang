/*

Messenger Backend Module

This module manages the messaging system for Slang. Database integrations and the 
socket listener is handled here.

Author: Surge
Version: 1.1

*/

//NOTES: Learn how to do websocket stuff

//Websocket listener service

extern crate websocket;

use std::thread;
// use websocket::header::Headers;
use websocket::sync::Server;
use websocket::OwnedMessage;

/*
message_type options
0: Basic Message (can include attachments and emojis (eventually))
1: Group Message (Stuff like member join/leave, announcements, etc.) //TODO: impl
2: System Message (Announcements, mainly)
*/
struct WSMessage {
	message_type: i32,
	message_author: i32,
	message_data: String
}

struct WSResponse {
	message_type: i32,
	message_data: String
}

//Grabbed from the websocket quickstart
pub fn start_websocket() {

	//Create the websocket server and bind it to port 9000
	let server = Server::bind("127.0.0.1:9000").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {

			//First check protocols, if it doesn't contain our custom protocol then reject
			if !request.protocols().contains(&"slang-ws".to_string()) {
				request.reject().unwrap();
				
				return;
			}

			//Create the client for our purposes
			let mut client = request.use_protocol("slang-ws").accept().unwrap();
			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			//TODO change this message to something Slang can actually use
			//Send a friendly hello!
			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			//Get the parts that we need to be able to send and receive messages between front and backend
			let (mut receiver, mut sender) = client.split().unwrap();

			//Now go through all the messages
			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				//Determine the type
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					OwnedMessage::Text(str) => {
						
					},
					_ => {} //Do nothing with the other types
					//TODO: message type handling
				}
			}
		});
	}
}