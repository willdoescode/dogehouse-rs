use crate::user::User;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewMessage {
	pub op: String,
	pub d: D,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct D {
	pub user_id: String,
	pub msg: Msg
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Msg {
	pub username: String,
	pub user_id: String,
	pub tokens: Vec<Tokens>,
	pub sent_at: String,
	pub is_whisper: bool,
	pub id: String,
	pub display_name: String,
	pub avatar_url: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
	pub v: String,
	pub t: String,
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
	pub(crate) user_id: &'a str,
	pub(crate) tokens: &'a Vec<Tokens>,
	pub(crate) is_whisper: bool,
	pub(crate) author: &'a str,
	pub(crate) content: &'a str,
}

// pub(crate) fn message_to_string(tokens: Vec<Tokens>) -> String {
// 	tokens.iter().map(|a| match a.t {
// 		String::from("mention") => format!("@{}", a.v),
// 		String::from("emote") => format!(":{}:", a.v),
// 		String::from("block") => format!("`{}`", a.v),
// 		_ => &a.v
// 	}).collect::<String>().trim().to_string()
// }