use crate::user::User;

pub struct Room<'a> {
	pub id: &'a str,
	pub creator_id: &'a str,
	pub name: &'a str,
	pub description: &'a str,
	pub is_private: bool,
	pub count: u32,
	pub users: Vec<User<'a>>,
}
