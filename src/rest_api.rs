use crate::{
    auto_import_models,
    correction_local_time::{CorrectionLocalTime, CorrectionTime},
    crypto::UserSettings,
    errors::{DzengiRestClientError, DzengiRestClientResult},
};

auto_import_models! {
    v1,
    v2
}

#[derive(Debug, Default, Clone)]
pub struct DzengiRestClient {
    demo: bool,
    settings: Option<UserSettings>,
    pub(crate) correction_time: CorrectionLocalTime,
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

    fn settings(&self) -> DzengiRestClientResult<&UserSettings> {
        self.settings
            .as_ref()
            .ok_or(DzengiRestClientError::NoneUserSettings)
    }

    pub fn correction_time(&self) -> &CorrectionLocalTime {
        &self.correction_time
    }

    pub fn correction_time_mut(&mut self) -> &mut CorrectionLocalTime {
        &mut self.correction_time
    }

    pub async fn calc_correction_with_server(&mut self) -> DzengiRestClientResult<()> {
        self.correction_time.with_correction(CorrectionTime::None);
        let local_time = self.correction_time.timestamp_now()?;
        let server_time = self.v2().server_time().await?.server_time;
        self.correction_time
            .with_correction(match local_time.cmp(&server_time) {
                std::cmp::Ordering::Equal => CorrectionTime::None,
                std::cmp::Ordering::Less => CorrectionTime::Add(server_time - local_time),
                std::cmp::Ordering::Greater => CorrectionTime::Sub(local_time - server_time),
            });
        Ok(())
    }

    pub fn v1(&self) -> Version1 {
        Version1::new(self)
    }

    pub fn v2(&self) -> Version2 {
        Version2::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::DzengiRestClient;

    #[tokio::test]
    async fn correction() {
        let mut rest = DzengiRestClient::new();

        rest.calc_correction_with_server().await.unwrap();
        println!("Correction: {:?}", rest.correction_time);
    }
}
