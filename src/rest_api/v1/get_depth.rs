use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::DepthResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct DepthRequest {
    pub symbol: String,
    pub limit: Option<usize>,
}

impl Version1<'_> {
    pub async fn depth(&self, request: DepthRequest) -> DzengiRestClientResult<DepthResponse> {
        let mut query = Query::<2>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/api/v1/depth", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::{DepthRequest, DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .v1()
            .depth(DepthRequest::new("BTC/USD_LEVERAGE".into()).with_limit(Some(10)))
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
