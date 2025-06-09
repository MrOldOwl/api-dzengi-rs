#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimits {
    pub interval: String,
    pub interval_num: i32,
    pub limit: i32,
    pub rate_limit_type: String,
}
