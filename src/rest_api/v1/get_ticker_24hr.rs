use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{SymbolRequest, Ticker24hr},
    switch_url,
};

impl Version1<'_> {
    pub async fn ticker_24hr(&self, request: SymbolRequest) -> DzengiRestClientResult<Ticker24hr> {
        let mut query = Query::<1>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/v1/ticker/24hr", self.demo))
            .query(&query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{models::SymbolRequest, rest_api::DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let mut rest = DzengiRestClient::new();

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .v1()
            .ticker_24hr(SymbolRequest::new("BTC/USD".into()))
            .await
            .unwrap();

        println!("{:?}", resp);
    }
}
