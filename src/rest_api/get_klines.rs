use super::DzengiRestClient;
use crate::{
    enums::Interval,
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::KlinesResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KlinesRequest {
    pub symbol: String,
    pub interval: Interval,
    pub kline_type: Option<String>,
    pub price_type: Option<String>,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}
impl KlinesRequest {
    pub fn new(symbol: String, interval: Interval) -> Self {
        Self {
            symbol,
            interval,
            kline_type: None,
            price_type: None,
            end_time: None,
            start_time: None,
            limit: None,
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn with_interval(mut self, interval: Interval) -> Self {
        self.interval = interval;
        self
    }

    pub fn with_kline_type(mut self, kline_type: Option<String>) -> Self {
        self.kline_type = kline_type;
        self
    }

    pub fn with_price_type(mut self, price_type: Option<String>) -> Self {
        self.price_type = price_type;
        self
    }

    pub fn with_limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_start_time(mut self, start_time: Option<u128>) -> Self {
        self.start_time = start_time;
        self
    }

    pub fn with_end_time(mut self, end_time: Option<u128>) -> Self {
        self.end_time = end_time;
        self
    }
}

impl DzengiRestClient {
    pub async fn klines(
        &self,
        request: KlinesRequest,
    ) -> DzengiRestClientResult<Vec<KlinesResponse>> {
        let mut query = Query::<7>::new();
        query.add(DefaultKeys::symbol(), request.symbol);
        query.add("interval", request.interval);
        query.add_option("priceType", request.price_type);
        query.add_option("type", request.kline_type);
        query.add_option("limit", request.limit);
        query.add_option("startTime", request.start_time);
        query.add_option("endTime", request.end_time);

        self.client
            .get(switch_url!("/api/v1/klines", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{
        enums::Interval,
        rest_api::{DzengiRestClient, KlinesRequest},
    };

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .klines(
                KlinesRequest::new("BTC/USD_LEVERAGE".into(), Interval::FiveMinutes)
                    .with_limit(Some(10)),
            )
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
