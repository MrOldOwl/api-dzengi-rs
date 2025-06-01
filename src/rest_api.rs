use crate::{auto_import_models, crypto::UserSettings};
use serde::de::DeserializeOwned;

auto_import_models! {
    get_account_info,
    get_server_time
}

#[derive(Debug, Default, Clone)]
pub struct DzengiRestClient {
    demo: bool,
    settings: Option<UserSettings>,
    client: reqwest::Client,
}

impl DzengiRestClient {
    pub fn new() -> Self {
        Self {
            demo: false,
            ..Default::default()
        }
    }

    pub fn base_url(mut self) -> Self {
        self.demo = false;
        self
    }

    pub fn demo_url(mut self) -> Self {
        self.demo = true;
        self
    }

    pub fn with_user_settings(mut self, settings: Option<UserSettings>) -> Self {
        self.settings = settings;
        self
    }

    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &[(&str, String)],
    ) -> Result<T, reqwest::Error> {
        self.client
            .get(url)
            .query(params)
            .send()
            .await?
            .json::<T>()
            .await
    }
}
