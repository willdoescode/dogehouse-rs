use crate::message::Message;
use crate::user::{User, PermAttrs};
use tungstenite::Message::Text;
use url::Url;
use std::collections::HashMap;
#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tungstenite::WebSocket;
use tungstenite::client::AutoStream;
use crate::HEARTBEAT_TIMEOUT;

pub trait Handler {
	fn on_ready(&self, _user: String);
	fn on_message(&self, _msg: &Message);
}

#[derive(Debug, Clone)]
pub struct Client<'t, T> where
	T: Handler
{
	token: &'t str,
	refresh_token: &'t str,
	handler: Option<T>,
	socket: Option<Arc<Mutex<WebSocket<AutoStream>>>>,
}

impl<'t, T> Client<'t, T> where
	T: Handler
{
	pub fn new(token: &'t str, refresh_token: &'t str) -> Self {
		Self {
			token,
			refresh_token,
			handler: None,
			socket: None,
		}
	}

	pub fn add_handler(mut self, handler: T) -> Self {
		self.handler = Some(handler);
		self
	}

	pub fn ask_to_speak(&self) {
		self.socket
			.as_ref()
			.unwrap()
			.lock()
			.unwrap()
			.write_message(tungstenite::Message::Text(
			json!({
				"op": "ask_to_speak",
				"d": {}
			}).to_string(),
		)).unwrap();
	}

	pub(crate) fn authenticate(&self, room: &str) {
		self.socket.as_ref().unwrap().lock().unwrap().write_message(
			tungstenite::Message::Text(
				json!(
					{
						"op": "auth",
						"d": {
							"accessToken": self.token,
							"refreshToken": self.refresh_token,
							"reconnectToVoice": false,
							"currentRoomId": room,
							"muted": true,
							"platform": "dogehouse-rs"
						}
					}
				).to_string()
			)
		).expect("Could not authenticate");
	}

	pub(crate) fn heartbeat(&self) {
		let socket = Arc::clone(&self.socket.as_ref().unwrap());
		std::thread::spawn(move || {
			loop {
				socket.lock().unwrap()
					.write_message(Text("ping".into())).unwrap();

				let message = socket.lock().unwrap()
					.read_message().unwrap();

				if message.is_text() || message.is_binary() { println!("{}", message.to_string()); }
				else if message.is_close() { panic!("Unable to authenticate"); }
				std::thread::sleep(std::time::Duration::from_secs(HEARTBEAT_TIMEOUT));
			}
		});
	}

	pub fn start(&mut self, room: &str) -> Result<(), Box<dyn std::error::Error>> {
		let handler = &self.handler.as_ref().expect("No handler provided");

		let (mut socket, _response) =
			tungstenite::connect(Url::parse(crate::API_URL)?)?;

		self.socket = Some(Arc::new(Mutex::new(socket)));

		let socket = Arc::clone(&self.socket.as_ref().unwrap());
		socket.lock().unwrap().write_message(tungstenite::Message::Text("ping".into()))?;

		self.authenticate(room);
		self.ask_to_speak();

		if socket.lock().unwrap().read_message().unwrap().is_close() { panic!("Failed to authenticate"); }
		// println!("{}", socket.lock().unwrap().read_message().expect("Error reading message"));
		self.heartbeat();

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
