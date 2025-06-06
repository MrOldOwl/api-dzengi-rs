use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::AccountResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountInfoRequest {
    pub show_zero_balance: bool,
    pub recv_window: Option<u64>,
}
impl Default for AccountInfoRequest {
    fn default() -> Self {
        Self {
            show_zero_balance: false,
            recv_window: None,
        }
    }
}
impl AccountInfoRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_zero(mut self, zero_balance: bool) -> Self {
        self.show_zero_balance = zero_balance;
        self
    }

    pub fn with_recv_window(mut self, recv_window: Option<u64>) -> Self {
        self.recv_window = recv_window;
        self
    }
}

impl DzengiRestClient {
    pub async fn account_info(
        &self,
        request: AccountInfoRequest,
    ) -> DzengiRestClientResult<AccountResponse> {
        let settings = self.settings()?;

        let mut query = Query::<3>::new();
        query.add(
            DefaultKeys::timestamp(),
            self.correction_time.timestamp_now()?,
        );
        query.add_option(DefaultKeys::recv_window(), request.recv_window);
        query.add("showZeroBalance", request.show_zero_balance);
        let signature = query.gen_signature(settings)?;

        self.client
            .get(switch_url!("/api/v1/account", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(query.as_slice())
            .query(&[(DefaultKeys::signature(), signature.as_str())])
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use env_file_reader::read_file;

    use crate::{
        crypto::UserSettings,
        rest_api::{AccountInfoRequest, DzengiRestClient},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest.account_info(AccountInfoRequest::new()).await.unwrap();
        println!("Info: {:?}", resp);
    }
}
