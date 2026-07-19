use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub _id: String,

    pub _username: String,

    pub _discriminator: String,

    pub _global_name: String,

    pub _display_name: String,

    pub avatar: Avatar,

    pub _has_custom_avatar: bool,

    pub _banner: Option<serde_json::Value>,

    pub _accent_color: Option<serde_json::Value>,

    pub _banner_color: Option<serde_json::Value>,

    pub _public_flags: i64,

    pub _has_public_badge_data: bool,

    pub _public_badge_labels: Vec<serde_json::Value>,

    pub _created_at: String,

    pub _mention: String,

    pub _is_bot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub _preview_url: String,

    pub full_size_url: String,

    pub _detected_format: String,

    pub _available_formats: Vec<serde_json::Value>,

    pub _is_animated: bool,

    pub _size_options: Vec<_SizeOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct _SizeOptions {
    pub _size: i64,
    pub _url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user: User,
}
