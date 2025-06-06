use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::AggTrades,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradesAggregatedRequest {
    pub symbol: String,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}
impl TradesAggregatedRequest {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            end_time: None,
            start_time: None,
            limit: None,
        }
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
    pub async fn trades_aggregated(
        &self,
        request: TradesAggregatedRequest,
    ) -> DzengiRestClientResult<Vec<AggTrades>> {
        let mut query = Query::<2>::new();
        query.add(DefaultKeys::symbol(), request.symbol);
        query.add_option("limit", request.limit);
        query.add_option("startTime", request.start_time);
        query.add_option("endTime", request.end_time);

        self.client
            .get(switch_url!("/api/v1/aggTrades", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::{DzengiRestClient, TradesAggregatedRequest};

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .trades_aggregated(
                TradesAggregatedRequest::new("BTC/USD_LEVERAGE".into()).with_limit(Some(10)),
            )
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
