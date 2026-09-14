use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Value,
    pub display_name: String,
    pub avatar: Avatar,
    pub has_custom_avatar: bool,
    pub banner: Value,
    pub accent_color: Value,
    pub banner_color: Value,
    pub public_flags: i64,
    pub has_public_badge_data: bool,
    pub public_badge_labels: Vec<Value>,
    pub created_at: String,
    pub mention: String,
    pub is_bot: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub preview_url: String,
    pub full_size_url: String,
    pub detected_format: String,
    pub available_formats: Vec<String>,
    pub is_animated: bool,
    pub size_options: Vec<SizeOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeOption {
    pub size: i64,
    pub url: String,
}
