use super::Version2;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{AggTrades, AggTradesRequest},
    switch_url,
};

impl Version2<'_> {
    pub async fn trades_aggregated(
        &self,
        request: AggTradesRequest,
    ) -> DzengiRestClientResult<Vec<AggTrades>> {
        let mut query = Query::<4>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/api/v2/aggTrades", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{models::AggTradesRequest, rest_api::DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .v2()
            .trades_aggregated(
                AggTradesRequest::new("BTC/USD_LEVERAGE".into()).with_limit(Some(10)),
            )
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
