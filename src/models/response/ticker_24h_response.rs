use super::Ticker24hr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24HResponse {
    pub tickers: Vec<Ticker24hr>,
}
