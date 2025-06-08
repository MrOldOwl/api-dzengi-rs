use crate::{errors::CorrectionTimeError, rest_api::DzengiRestClient};
use zeroize::Zeroizing;

pub struct DefaultKeys;

impl DefaultKeys {
    pub fn timestamp(
        client: &DzengiRestClient,
    ) -> Result<(&'static str, String), CorrectionTimeError> {
        Ok((
            "timestamp",
            client.correction_time.timestamp_now()?.to_string(),
        ))
    }

    pub fn api_key() -> &'static str {
        "X-MBX-APIKEY"
    }

    pub fn signature(value: &Zeroizing<String>) -> [(&'static str, &str); 1] {
        [("signature", value.as_str())]
    }
}
