use super::Version2;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, Query},
    models::{KlinesRequest, KlinesResponse},
    switch_url,
};

impl Version2<'_> {
    pub async fn klines(
        &self,
        request: KlinesRequest,
    ) -> DzengiRestClientResult<Vec<KlinesResponse>> {
        let mut query = Query::<7>::new();
        request.fill_query(&mut query);

        self.client
            .get(switch_url!("/v2/klines", self.demo))
            .query(query.as_slice())
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{enums::Interval, models::KlinesRequest, rest_api::DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();

        let resp = rest
            .v2()
            .klines(
                KlinesRequest::new("BTC/USD_LEVERAGE".into(), Interval::FiveMinutes)
                    .with_limit(Some(10)),
            )
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
