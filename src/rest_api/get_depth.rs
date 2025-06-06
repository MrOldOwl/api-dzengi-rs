use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::DepthResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthRequest {
    pub symbol: String,
    pub limit: Option<usize>,
}
impl DepthRequest {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            limit: None,
        }
    }

    pub fn with_limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }
}

impl DzengiRestClient {
    pub async fn depth(&self, request: DepthRequest) -> DzengiRestClientResult<DepthResponse> {
        let mut query = Query::<2>::new();
        query.add(DefaultKeys::symbol(), request.symbol);
        query.add_option("limit", request.limit);

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
            .depth(DepthRequest::new("BTC/USD_LEVERAGE".into()).with_limit(Some(10)))
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
