use crate::{enums::Interval, help::Query};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct KlinesRequest {
    pub symbol: String,
    pub interval: Interval,
    #[serde(rename = "type")]
    pub kline_type: Option<String>,
    pub price_type: Option<String>,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}
