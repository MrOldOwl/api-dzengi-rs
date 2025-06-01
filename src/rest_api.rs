use crate::{
    auto_import_models,
    crypto::UserSettings,
    enums::CorrectionLocalTime,
    errors::{DzengiRestClientError, DzengiRestClientResult},
    help::timestamp_now,
};

auto_import_models! {
    get_account_info,
    get_server_time
}

#[derive(Debug, Default, Clone)]
pub struct DzengiRestClient {
    demo: bool,
    settings: Option<UserSettings>,
    correction_time: CorrectionLocalTime,
    client: reqwest::Client,
}

impl DzengiRestClient {
    pub fn new() -> Self {
        Self::default()
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

    pub async fn with_correction_time(mut self, correction_time: CorrectionLocalTime) -> Self {
        self.correction_time = correction_time;
        self
    }

    pub async fn with_correction_time_req(&mut self) -> DzengiRestClientResult<()> {
        let local_time = timestamp_now(CorrectionLocalTime::None)?;
        let server_time = self.server_time().await?.server_time;
        self.correction_time = match local_time.cmp(&server_time) {
            std::cmp::Ordering::Equal => CorrectionLocalTime::None,
            std::cmp::Ordering::Less => CorrectionLocalTime::Add(server_time - local_time),
            std::cmp::Ordering::Greater => CorrectionLocalTime::Sub(local_time - server_time),
        };
        Ok(())
    }

    fn settings(&self) -> DzengiRestClientResult<&UserSettings> {
        self.settings
            .as_ref()
            .ok_or(DzengiRestClientError::NoneUserSettings)
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::DzengiRestClient;

    #[tokio::test]
    async fn correction() {
        let mut rest = DzengiRestClient::new();

        rest.with_correction_time_req().await.unwrap();
        println!("Correction: {:?}", rest.correction_time);
    }
}
