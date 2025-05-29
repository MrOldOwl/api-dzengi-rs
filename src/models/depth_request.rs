#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthRequest {
    pub limit: Option<i32>,
    pub symbol: String,
}
