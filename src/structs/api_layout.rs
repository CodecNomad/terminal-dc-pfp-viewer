use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub _id: String,
    pub _username: String,
    pub _discriminator: String,
    pub _global_name: String,
    pub _display_name: String,
    pub _avatar_url: String,
    pub avatar_download_url: String,
    pub _avatar_webp_url: String,
    pub _avatar_animated: bool,
    pub _banner_url: Option<String>,
    pub _banner_download_url: Option<String>,
    pub _banner_animated: bool,
    pub _accent_color: Option<String>,
    pub _banner_color: Option<String>,
    pub _public_flags: i32,
    pub _created_at: String,
    pub _mention: String,
    pub _is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user: User,
}
