use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::TradingFeesResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct TradingFeesRequest {
    pub symbol: String,
}

impl DzengiRestClient {
    pub async fn trading_fees(
        &self,
        request: TradingFeesRequest,
    ) -> DzengiRestClientResult<Vec<TradingFeesResponse>> {
        let mut query = Query::<1>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/api/v1/tradingFees", self.demo))
            .query(&query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::{DzengiRestClient, TradingFeesRequest};

    #[tokio::test]
    async fn test() {
        let mut rest = DzengiRestClient::new();

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .trading_fees(TradingFeesRequest::new("BTC/USD".into()))
            .await
            .unwrap();

        println!("{:?}", resp);
    }
}
