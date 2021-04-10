use crate::user::User;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Message<'a> {
	pub(crate) message_id: &'a str,
	pub(crate) tokens: Vec<HashMap<&'a str, &'a str>>,
	pub(crate) is_whisper: bool,
	pub(crate) author: User<'a>,
	pub(crate) content: &'a str,
}
