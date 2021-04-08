pub(crate) trait Handler {
	fn on_ready(&self) {}
	fn on_message(&self) {}
}

pub(crate) struct Client<'t, T> where T: Handler {
	token: &'t str,
	refresh_token: &'t str,
	handler: Option<T>,
}

impl<'t, T> Client<'t, T> where T: Handler {
	pub(crate) fn new(token: &'t str, refresh_token: &'t str) -> Self {
		Self {
			token,
			refresh_token,
			handler: None,
		}
	}

	pub(crate) fn add_handler(mut self, handler: T) -> Self {
		self.handler = Some(handler);
		self
	}
}
