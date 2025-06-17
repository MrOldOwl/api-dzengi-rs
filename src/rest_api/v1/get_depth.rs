use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{DepthRequest, DepthResponse},
    switch_url,
};

impl Version1<'_> {
    pub async fn depth(&self, request: DepthRequest) -> DzengiRestClientResult<DepthResponse> {
        let mut query = Query::<2>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/v1/depth", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{models::DepthRequest, rest_api::DzengiRestClient};

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
