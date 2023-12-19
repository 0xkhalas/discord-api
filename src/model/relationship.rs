use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::User::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub nickname: Value,
    pub user: User,
    pub since: Option<String>,
}
