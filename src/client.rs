#![allow(unused_imports)]

use crate::message::Message;
use tungstenite::ClientHandshake;
use crate::user::{User, PermAttrs};
use tungstenite::Message::Text;
use url::Url;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

pub trait Handler {
	fn on_ready(&self, _user: String);
	fn on_message(&self, _msg: &Message);
}

#[derive(Debug, Copy, Clone)]
pub struct Client<'t, T> where
	T: Handler
{
	token: &'t str,
	refresh_token: &'t str,
	handler: Option<T>,
}

impl<'t, T> Client<'t, T> where
	T: Handler
{
	pub fn new(token: &'t str, refresh_token: &'t str) -> Self {
		Self {
			token,
			refresh_token,
			handler: None,
		}
	}

	pub fn add_handler(mut self, handler: T) -> Self {
		self.handler = Some(handler);
		self
	}

	pub fn start(&mut self) -> Result<(), &'static str> {
		let handler = self.handler.as_ref().expect("No handler provided");
		let (mut socket, _response) = tungstenite::connect(
			Url::parse(crate::API_URL).unwrap()
		).expect("Could not connect");
		let mut socket = Arc::new(Mutex::new(socket));
		{
			let socket = Arc::clone(&socket);
			std::thread::spawn(move || {
				loop {
					socket.lock().unwrap().write_message(Text("ping".into())).unwrap();
					let message = socket.lock().unwrap().read_message().expect("Error reading socket message");
					if message.is_text() || message.is_binary() { println!("{}", message.to_string()); }
					else if message.is_close() { panic!("Unable to authenticate"); }
					std::thread::sleep(std::time::Duration::from_secs(8));
				}
			});
		}

		let socket = Arc::clone(&socket);
		socket.lock().unwrap().write_message(
			tungstenite::Message::Text(
				json!(
					{
						"op": "auth",
						"accessToken": self.token,
						"refreshToken": self.refresh_token,
						"reconnectToVoice": false,
						"currentRoomId": "3daf5a80-5b0a-4dde-9527-9db1f7f13755",
						"muted": true,
						"platform": "dogehouse-rs"
					}
				).to_string()
			)
		).expect("Could not write message");
		println!("{}", socket.lock().unwrap().read_message().expect("Error reading message"));

		handler.on_ready(String::from("Bot is ready"));
		loop {
			handler.on_message(&Message{
					message_id: "will",
					author: User{
						user_id: "will",
						username: "will",
						display_name: "will",
						avatar_url: "will",
						bio: "will",
						last_seen: "will",
						online: false,
						following: false,
						perms: PermAttrs {
							asked_to_speak: false,
							is_mod: false,
							is_admin: false,
							is_speaker: false
						},
						num_followers: 0,
						num_following: 0,
						follows_me: false,
						current_room_id: "will"
					},
					content: "hello world",
					is_whisper: false,

					tokens: vec![HashMap::new()]
				}
			);
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
		unreachable!();

		Ok(())
	}
}
