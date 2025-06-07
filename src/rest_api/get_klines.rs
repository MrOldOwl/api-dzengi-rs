use super::DzengiRestClient;
use crate::{
    enums::Interval,
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::KlinesResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct KlinesRequest {
    pub symbol: String,
    pub interval: Interval,
    pub kline_type: Option<String>,
    pub price_type: Option<String>,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}

impl DzengiRestClient {
    pub async fn klines(
        &self,
        request: KlinesRequest,
    ) -> DzengiRestClientResult<Vec<KlinesResponse>> {
        let mut query = Query::<7>::new();
        request.fill_query(&mut query);

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
