#[allow(dead_code)]
pub mod test {
	pub(crate) const OPERATOR: &'static str = "test:operator";
}

#[allow(dead_code)]
pub mod user {
	pub(crate) const CREATE_BOT: &'static str = "user:create_bot";
	pub(crate) const BAN: &'static str = "user:ban";
	pub(crate) const BLOCK: &'static str = "user:block";
	pub(crate) const UNBLOCK: &'static str = "user:unblock";
	pub(crate) const FOLLOW: &'static str = "user:follow";
	pub(crate) const GET_FOLLOWING: &'static str = "user:get_following";
	pub(crate) const GET_FOLLOWERS: &'static str = "user:get_followers";
	pub(crate) const UPDATE: &'static str = "user:update";
	pub(crate) const GET_INFO: &'static str = "user:get_info";
	pub(crate) const GET_RELATIONSHIP: &'static str = "user:get_relationship";
	pub(crate) const UNFOLLOW: &'static str = "user:get_unfollow";
}

#[allow(dead_code)]
pub mod room {
	pub(crate) const INVITE: &'static str = "room:invite";
	pub(crate) const UPDATE: &'static str = "room:update";
	pub(crate) const GET_INVITE_LIST: &'static str = "room:get_invite_list";
	pub(crate) const LEAVE: &'static str = "room:leave";
	pub(crate) const BAN: &'static str = "room:ban";
	pub(crate) const SET_ROLE: &'static str = "room:set_role";
	pub(crate) const SET_AUTH: &'static str = "room:set_auth";
	pub(crate) const JOIN: &'static str = "room:join";
	pub(crate) const GET_BANNED_USERS: &'static str = "room:get_banned_users";
	pub(crate) const UPDATE_SCHEDULED: &'static str = "room:update_scheduled";
	pub(crate) const DELETE_SCHEDULED: &'static str = "room:delete_scheduled";
	pub(crate) const CREATE: &'static str = "room:create";
	pub(crate) const CREATE_SCHEDULED: &'static str = "room:create_scheduled";
	pub(crate) const UNBAN: &'static str = "room:unban";
	pub(crate) const GET_INFO: &'static str = "room:get_info";
	pub(crate) const GET_TOP: &'static str = "room:get_top";
	pub(crate) const SET_ACTIVE_SPEAKER: &'static str = "room:set_active_speaker";
	pub(crate) const MUTE: &'static str = "room:mute";
	pub(crate) const DEAFEN: &'static str = "room:deafen";
	pub(crate) const GET_SCHEDULED: &'static str = "room:get_scheduled";
}

#[allow(dead_code)]
pub mod chat {
	pub(crate) const BAN: &'static str = "chat:ban";
	pub(crate) const UNBAN: &'static str = "chat:unban";
	pub(crate) const SEND_MSG: &'static str = "chat:send_msg";
	pub(crate) const DELETE: &'static str = "chat:delete";
}

#[allow(dead_code)]
pub mod auth {
	pub(crate) const REQUEST: &'static str = "auth:request";
}

#[allow(dead_code)]
pub mod misc {
	pub(crate) const DELETE: &'static str = "misc:search";
}
