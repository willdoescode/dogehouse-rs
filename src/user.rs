use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TopUserWrapper {
    pub(crate) d: MidUserWrapper,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct MidUserWrapper {
    pub(crate) user: User,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub avatar_url: String,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
    pub current_room_id: Option<String>,
    pub display_name: String,
    pub follows_you: Option<bool>,
    pub i_blocked_them: Option<bool>,
    pub id: String,
    pub last_online: Option<String>,
    pub num_followers: usize,
    pub num_following: usize,
    pub online: bool,
    pub username: String,
}
