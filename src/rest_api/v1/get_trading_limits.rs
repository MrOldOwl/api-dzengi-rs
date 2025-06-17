use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{SymbolRequest, TradingLimitsResponse},
    switch_url,
};

impl Version1<'_> {
    pub async fn trading_limits(
        &self,
        request: SymbolRequest,
    ) -> DzengiRestClientResult<Vec<TradingLimitsResponse>> {
        let mut query = Query::<1>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/v1/tradingLimits", self.demo))
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
            .trading_limits(SymbolRequest::new("BTC/USD".into()))
            .await
            .unwrap();

        println!("{:?}", resp);
    }
}
