use serde::{Serialize, Deserialize};

use super::avatar_decoration_data::AvatarDecorationData;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    pub id: String,
    pub username: String,
    #[serde(rename = "global_name")]
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub discriminator: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    pub bot: Option<bool>,
    pub system: Option<bool>,
}