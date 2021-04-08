pub(crate) trait Handler {}

pub(crate) struct Client<'t, T> where T: Handler {
	token: &'t String,
	refresh_token: &'t String,
	handler: Option<T>,
}

impl<'t, T> Client<'t, T> where T: Handler {
	pub(crate) fn new(token: &'t String, refresh_token: &'t String) -> Self {
		Self {
			token,
			refresh_token,
			handler: None,
		}
	}
}

#[cfg(test)]
mod tests;

