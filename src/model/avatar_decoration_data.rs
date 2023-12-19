use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDecorationData {
    pub asset: String,
    #[serde(rename = "sku_id")]
    pub sku_id: String,
}

