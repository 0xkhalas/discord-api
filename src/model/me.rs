use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    #[serde(rename = "premium_type")]
    pub premium_type: i64,
    pub flags: i64,
    pub banner: Value,
    #[serde(rename = "accent_color")]
    pub accent_color: i64,
    #[serde(rename = "global_name")]
    pub global_name: String,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Value,
    #[serde(rename = "banner_color")]
    pub banner_color: String,
    #[serde(rename = "mfa_enabled")]
    pub mfa_enabled: bool,
    pub locale: String,
    pub email: String,
    pub verified: bool,
    pub phone: String,
    #[serde(rename = "nsfw_allowed")]
    pub nsfw_allowed: bool,
    #[serde(rename = "linked_users")]
    pub linked_users: Vec<Value>,
    #[serde(rename = "purchased_flags")]
    pub purchased_flags: i64,
    pub bio: String,
    #[serde(rename = "authenticator_types")]
    pub authenticator_types: Vec<i64>,
}
