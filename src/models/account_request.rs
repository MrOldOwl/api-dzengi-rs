#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountRequest {
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    pub show_zero_balance: Option<bool>,
    #[validate(length(min = 1))]
    pub signature: String,
    pub timestamp: i64,
}
