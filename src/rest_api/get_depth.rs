use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys},
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
        let url = switch_url!("/api/v1/depth", self.demo);

        let mut req = self
            .client
            .get(url)
            .query(&[(DefaultKeys::symbol(), request.symbol)]);

        if let Some(limit) = request.limit {
            req = req.query(&[("limit", limit.to_string())])
        }

        req.send_and_json().await
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
