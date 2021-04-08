use tungstenite::{connect, Message};
use url::Url;

pub trait Handler {
	fn on_ready(&self, user: String) {}
	fn on_message(&self, msg: String) {}
}

#[derive(Debug, Copy, Clone)]
pub struct Client<'t, T> where T: Handler {
	token: &'t str,
	refresh_token: &'t str,
	handler: Option<T>,
}

impl<'t, T> Client<'t, T> where T: Handler {
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

	pub fn start(&mut self) -> Result<(), String> {
		let handler = self.handler.as_ref().expect("No handler provided");
		std::thread::spawn(|| {
			let (mut socket, _response) = connect(
				Url::parse(crate::API_URL).unwrap()
			).expect("Could not connect");

			loop {
				socket.write_message(Message::Text("ping".into())).unwrap();
				std::thread::sleep(std::time::Duration::from_secs(8));
			}
		});

		handler.on_ready(String::from("Bot is ready"));
		loop {
			handler.on_message(String::from("message thing"));
			std::thread::sleep(std::time::Duration::from_secs(1));
		}

		Ok(())
	}
}
