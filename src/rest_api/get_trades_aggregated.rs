use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::AggTrades,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct TradesAggregatedRequest {
    pub symbol: String,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}

impl DzengiRestClient {
    pub async fn trades_aggregated(
        &self,
        request: TradesAggregatedRequest,
    ) -> DzengiRestClientResult<Vec<AggTrades>> {
        let mut query = Query::<4>::new();
        request.fill_query(&mut query);

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
