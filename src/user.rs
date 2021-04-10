#[derive(Debug, Copy, Clone)]
pub struct User<'a> {
	pub(crate) user_id: &'a str,
	pub(crate) username: &'a str,
	pub(crate) display_name: &'a str,
	pub(crate) avatar_url: &'a str,
	pub(crate) bio: &'a str,
	pub(crate) last_seen: &'a str,
	pub(crate) online: bool,
	pub(crate) following: bool,
	pub(crate) perms: PermAttrs,
	pub(crate) num_followers: u32,
	pub(crate) num_following: u32,
	pub(crate) follows_me: bool,
	pub(crate) current_room_id: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct PermAttrs {
	pub(crate) asked_to_speak: bool,
	pub(crate) is_mod: bool,
	pub(crate) is_admin: bool,
	pub (crate) is_speaker: bool,
}

// impl<'a> User<'a> {
// 	pub fn from_id(id: &'a str) -> Option<Self> {
// 		Some(Self {
// 			username: "test",
// 			id
// 		})
// 	}
// }
