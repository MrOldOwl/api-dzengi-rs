#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OvernightRate {
    pub long_rate: f64,
    pub short_rate: f64,
}
