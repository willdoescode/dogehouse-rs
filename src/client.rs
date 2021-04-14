use crate::message::{Message, NewMessage, Tokens};
use crate::user::{User, PermAttrs};
use tungstenite::Message::Text;
use url::Url;
use std::collections::HashMap;
#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex, RwLock};
use tungstenite::WebSocket;
use tungstenite::client::AutoStream;
use crate::HEARTBEAT_TIMEOUT;
use std::time::{Instant, Duration};

pub trait Handler {
	fn on_ready(&self, _user: String);
	fn on_message(&self, _msg: &Message);
}

#[derive(Debug, Clone)]
pub struct Client<T> where
	T: Clone + Handler + Send + Sync + 'static
{
	token: String,
	refresh_token: String,
	handler: Option<Arc<T>>,
	socket: Option<Arc<RwLock<WebSocket<AutoStream>>>>,
	message_queue: Arc<RwLock<Vec<String>>>,
	time_since: Arc<RwLock<Instant>>,
	time_since_heart_beat: Arc<RwLock<Instant>>,
}

impl<T> Client<T> where
	T: Clone + Handler + Send + Sync + 'static
{
	pub fn new(token: String, refresh_token: String) -> Self {
		Self {
			token,
			refresh_token,
			handler: None,
			socket: None,
			message_queue: Arc::new(RwLock::new(Vec::new())),
			time_since: Arc::new(RwLock::new(Instant::now())),
			time_since_heart_beat: Arc::new(RwLock::new(Instant::now())),
		}
	}

	pub fn add_handler(mut self, handler: T) -> Self {
		self.handler = Some(Arc::new(handler));
		self
	}

	pub fn ask_to_speak(&self) {
		self.socket
			.as_ref()
			.unwrap()
			.write()
			.unwrap()
			.write_message(tungstenite::Message::Text(
			json!({
				"op": "ask_to_speak",
				"d": {}
			}).to_string(),
		)).unwrap();
	}


	pub(crate) fn authenticate(&self, room: &str) {
		self.socket.as_ref().unwrap().write().unwrap().write_message(
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

	pub fn send_message(&self, msg: &String) {
		let socket = Arc::clone(&self.socket.as_ref().unwrap());

		let mut tokens: Vec<Tokens> = Vec::new();
		for s in msg.split_whitespace() {
			tokens.push(Tokens {
				v: s.to_string(),
				t: "text".to_string()
			})
		}

		let json_msg = json!({
			"op": "send_room_chat_msg",
			"d": {
				"tokens": tokens,
			}
		}).to_string();

		println!("{:?}", &json_msg);

		socket.write().unwrap().write_message(Text(json_msg)).unwrap();
	}

	pub(crate) fn start_message_loop(self: Arc<Self>) {
		let socket = Arc::clone(&self.socket.as_ref().unwrap());
		let time_since_heart_beat = Arc::clone(&self.time_since_heart_beat);
		let message_queue = Arc::clone(&self.message_queue);
		let time_since = Arc::clone(&self.time_since);
		std::thread::spawn(move || {
			loop {
				if time_since_heart_beat.read().unwrap().elapsed().as_secs() >= 8 && socket.read().unwrap().can_write() {
					socket.write().unwrap()
						.write_message(Text("ping".into())).unwrap();

					let message = socket.write().unwrap()
						.read_message().unwrap();

					println!("{}", message.to_string());
					if message.is_close() { panic!("Unable to authenticate"); }
					*time_since_heart_beat.write().unwrap() = Instant::now();
				}

				if !self.message_queue.read().unwrap().is_empty() &&
					self.time_since.read().unwrap().elapsed().as_secs() > 1 &&
					socket.read().unwrap().can_write()
				{
					println!("{:?}", message_queue.read().unwrap());
					self.send_message(self.message_queue.read().unwrap().get(0).unwrap());
					*message_queue.write().unwrap() = message_queue.read().unwrap().as_slice()[1..].to_vec();
					println!("{:?}", self.message_queue.read().unwrap());
					*time_since.write().unwrap() = Instant::now();
				}
			}
		});
	}

	pub(crate) fn start_loop(&self) {
		let socket = Arc::clone(&self.socket.as_ref().unwrap());
		loop {
			if socket.read().unwrap().can_read() {
				let message = socket.write().unwrap().read_message().unwrap();
				if message.is_text() || message.is_binary() {
					println!("{}", message.to_string());
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

						self.message_queue.write().unwrap().push(format!("{} said {}", &new_message.d.msg.username, &msg_str));
						self.handler.as_ref().unwrap().on_message(&Message {
							user_id: &new_message.d.user_id,
							tokens: &new_message.d.msg.tokens,
							is_whisper: new_message.d.msg.is_whisper,
							author: &new_message.d.msg.username,
							content: &msg_str
						});
					}
				}
			}
		}
	}

	pub fn start(mut self, room: &str) -> Result<(), Box<dyn std::error::Error>> {
		let (mut socket, _response) =
			tungstenite::connect(Url::parse(crate::API_URL)?)?;

		self.socket = Some(Arc::new(RwLock::new(socket)));
		let socket = Arc::clone(&self.socket.as_ref().unwrap());

		self.authenticate(room);

		if socket.write().unwrap()
			.read_message()
			.unwrap()
			.is_close() { panic!("Failed to authenticate"); }

		Arc::new(self.clone()).start_message_loop();
		self.start_loop();

		loop {}

		Ok(())
	}
}
