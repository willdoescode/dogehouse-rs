use crate::message::Message;
use crate::user::{User, PermAttrs};
use tungstenite::Message::Text;
use url::Url;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;

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
		let (socket, _response) = tungstenite::connect(
			Url::parse(crate::API_URL).unwrap()
		).expect("Could not connect");

		let s = Arc::new(Mutex::new(socket));
		let mut socket = Arc::clone(&s);

		std::thread::spawn(move || {
			loop {
				let mut socket = socket.lock().unwrap();

				socket.write_message(Text("ping".into())).unwrap();
				let message = socket.read_message().expect("Error reading socket message");
				if message.is_text() || message.is_binary() { println!("{}", message.to_string()); }
				else if message.is_close() { panic!("Unable to authenticate"); }
				// std::thread::sleep(std::time::Duration::from_secs(8));
			}
		});

		let mut socket = Arc::clone(&s);
		let mut socket = socket.lock().unwrap();
		socket.read_message().unwrap();

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

		Ok(())
	}
}
