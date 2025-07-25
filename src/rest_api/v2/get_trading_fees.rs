use super::Version2;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{SymbolRequest, TradingFeesResponse},
    switch_url,
};

impl Version2<'_> {
    pub async fn trading_fees(
        &self,
        request: SymbolRequest,
    ) -> DzengiRestClientResult<Vec<TradingFeesResponse>> {
        let mut query = Query::<1>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/v2/tradingFees", self.demo))
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
            .v2()
            .trading_fees(SymbolRequest::new("BTC/USD".into()))
            .await
            .unwrap();

        println!("{:?}", resp);
    }
}
