use crate::errors::CryptoError;
use hmac::{Hmac, Mac};
use secstr::SecStr;
use sha2::Sha256;
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSettings {
    pub(crate) api_key: Zeroizing<String>,
    pub(crate) secret: SecStr,
}

impl UserSettings {
    pub fn new(api_key: impl AsRef<str>, secret: impl AsRef<str>) -> Self {
        Self::from_sec(api_key.as_ref().into(), secret.as_ref().into())
    }

    pub fn from_sec(api_key: String, secret: SecStr) -> Self {
        Self {
            api_key: api_key.into(),
            secret,
        }
    }

    pub fn gen_signature(
        &self,
        query_pairs: &[(impl AsRef<str>, impl AsRef<str>)],
    ) -> Result<Zeroizing<String>, CryptoError> {
        if query_pairs.len() == 0 {
            return Err(CryptoError::ParamsEmpty);
        }

        let mut query_combine = query_pairs.iter().map(Self::formatting);

        let mut query_string = query_combine.next().unwrap();
        while let Some(pair) = query_combine.next() {
            query_string.push('&');
            query_string.push_str(pair.as_str());
        }

        let mut mac = {
            let slice = self.secret.unsecure();
            <Hmac<Sha256> as Mac>::new_from_slice(slice)?
        };

        mac.update(query_string.as_bytes());

        Ok(hex::encode(mac.finalize().into_bytes()).into())
    }

    fn formatting(value: &(impl AsRef<str>, impl AsRef<str>)) -> String {
        let key = value.0.as_ref();
        let value = value.1.as_ref();
        if key == "symbol" {
            format!("{}={}", key, value.replace("/", "%2F"))
        } else {
            format!("{}={}", key, value)
        }
    }
}
