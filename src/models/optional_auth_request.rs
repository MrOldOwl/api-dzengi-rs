#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct OptionalAuthRequest {
    pub api_key: Option<String>,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    pub signature: Option<String>,
    pub timestamp: Option<i64>,
}
