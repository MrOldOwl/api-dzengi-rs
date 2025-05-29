use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeResponse {
    pub error_code: Option<String>,
    pub subscriptions: HashMap<String, String>,
}
