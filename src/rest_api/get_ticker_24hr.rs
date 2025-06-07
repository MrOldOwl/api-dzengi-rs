use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{CurrencyDtoResponse, Ticker24hr},
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct Ticker24hrRequest {
    pub symbol: String,
}

impl DzengiRestClient {
    pub async fn ticker_24hr(
        &self,
        request: Ticker24hrRequest,
    ) -> DzengiRestClientResult<Ticker24hr> {
        let mut query = Query::<1>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/api/v1/ticker/24hr", self.demo))
            .query(&query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::{DzengiRestClient, Ticker24hrRequest};

    #[tokio::test]
    async fn test() {
        let mut rest = DzengiRestClient::new();

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .ticker_24hr(Ticker24hrRequest::new("BTC/USD".into()))
            .await
            .unwrap();

        println!("{:?}", resp);
    }
}
