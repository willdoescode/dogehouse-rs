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

	pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let handler = self.handler.unwrap_or_else(Err("No handler provided"));
		handler.on_ready(String::from("Bot is ready"));
		loop {
			handler.on_message(String::from("message thing"));
			std::thread::sleep(std::time::Duration::from_secs(1));
		}

		Ok(())
	}
}
