use super::Version2;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::TradingPositionHistoryResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct TradingPositionsHistoryRequest {
    pub symbol: String,
    pub recv_window: Option<u64>,
    pub limit: Option<usize>,
    pub from: Option<u128>,
    pub to: Option<u128>,
}

impl Version2<'_> {
    pub async fn trading_positions_history(
        &self,
        request: TradingPositionsHistoryRequest,
    ) -> DzengiRestClientResult<TradingPositionHistoryResponse> {
        let settings = self.settings()?;

        let mut query = Query::<6>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/api/v2/tradingPositionsHistory", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(query.as_slice())
            .query(&DefaultKeys::signature(&signature))
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use env_file_reader::read_file;

    use crate::{
        crypto::UserSettings,
        rest_api::{DzengiRestClient, TradingPositionsHistoryRequest},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        // TODO: req invalid api
        let resp = rest
            .v2()
            .trading_positions_history(TradingPositionsHistoryRequest::new("LTC/BTC".into()))
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
