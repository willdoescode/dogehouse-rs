use crate::message::{Message, NewMessage};
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
pub struct Client<T> where
	T: Handler + Sync
{
	token: String,
	refresh_token: String,
	handler: Option<T>,
	socket: Option<Arc<Mutex<WebSocket<AutoStream>>>>,
}

impl<T> Client<T> where
	T: Handler + Sync
{
	pub fn new(token: String, refresh_token: String) -> Self {
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

	// TODO: Send message functionality
	// pub fn send_message(msg: &str) {
	//
	// }

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

				if message.is_close() { panic!("Unable to authenticate"); }
				std::thread::sleep(std::time::Duration::from_secs(HEARTBEAT_TIMEOUT));
			}
		});
	}

	pub(crate) fn start_loop(&self) {
		let socket = Arc::clone(&self.socket.as_ref().unwrap());
		loop {
			let message = socket.lock().unwrap().read_message().unwrap();
			if message.is_text() || message.is_binary() {
				if message.to_string().starts_with("{\"op\":\"new_chat_msg\"") {
					// println!("{}", message.to_string());
					let new_message: NewMessage = serde_json::from_str(&message.to_string()).unwrap();
					let mut msg_str = String::new();
					for (i, token) in new_message.d.msg.tokens.iter().enumerate() {
						if i != 0 {
							msg_str.push_str(&format!(" {}", token.v));
						} else {
							msg_str.push_str(&token.v);
						}
					}
					self.handler.as_ref().unwrap().on_message(&Message {
						user_id: &new_message.d.user_id,
						tokens: &new_message.d.msg.tokens,
						is_whisper: new_message.d.msg.is_whisper,
						author: &new_message.d.msg.username,
						content: &msg_str
					})
				}
			}
		}
	}

	pub fn start(&mut self, room: &str) -> Result<(), Box<dyn std::error::Error>> {
		let (mut socket, _response) =
			tungstenite::connect(Url::parse(crate::API_URL)?)?;

		self.socket = Some(Arc::new(Mutex::new(socket)));
		let socket = Arc::clone(&self.socket.as_ref().unwrap());

		self.authenticate(room);

		if socket.lock().unwrap()
			.read_message()
			.unwrap()
			.is_close() { panic!("Failed to authenticate"); }

		self.heartbeat();
		self.start_loop();

		loop {}

		Ok(())
	}
}
