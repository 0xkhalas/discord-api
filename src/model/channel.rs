use serde::{Serialize, Deserialize};

use super::recipient::Recipient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub last_message_id: Option<String>,
    pub flags: i64,
    pub recipients: Vec<Recipient>,
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
}