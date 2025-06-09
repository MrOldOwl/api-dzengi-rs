#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OHLCBar {
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    pub interval: String,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub bar_type: String,
}
