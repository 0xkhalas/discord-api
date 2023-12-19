use serde::{Serialize, Deserialize};

use super::recipient::Recipient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "last_message_id")]
    pub last_message_id: Option<String>,
    pub flags: i64,
    pub recipients: Vec<Recipient>,
    #[serde(rename = "last_pin_timestamp")]
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "owner_id")]
    pub owner_id: Option<String>,
}