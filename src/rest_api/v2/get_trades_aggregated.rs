use super::Version2;
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

impl Version2<'_> {
    pub async fn trades_aggregated(
        &self,
        request: TradesAggregatedRequest,
    ) -> DzengiRestClientResult<Vec<AggTrades>> {
        let mut query = Query::<4>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/api/v2/aggTrades", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::{AggTradesRequest, DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .v2()
            .trades_aggregated(
                AggTradesRequest::new("BTC/USD_LEVERAGE".into()).with_limit(Some(10)),
            )
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
