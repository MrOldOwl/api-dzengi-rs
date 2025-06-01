use crate::errors::CryptoError;
use hmac::{Hmac, Mac};
use secstr::SecStr;
use sha2::Sha256;
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestParam {
    pub key: &'static str,
    pub value: String,
}

impl RequestParam {
    pub fn new(key: &'static str, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSettings {
    pub(crate) api_key: Zeroizing<String>,
    pub(crate) secret: SecStr,
}

impl UserSettings {
    pub fn new(api_key: &str, secret: &str) -> Self {
        Self::from_sec(api_key.into(), secret.into())
    }

    pub fn from_sec(api_key: String, secret: SecStr) -> Self {
        Self {
            api_key: api_key.into(),
            secret,
        }
    }

    pub fn generate_signature(
        &self,
        params: &mut [RequestParam],
    ) -> Result<Zeroizing<String>, CryptoError> {
        if params.len() == 0 {
            return Err(CryptoError::ParamsEmpty);
        }
        params.sort_by(|lhs, rhs| lhs.key.cmp(rhs.key));

        let mut pairs = params.iter().map(|x| format!("{}={}", x.key, x.value));

        let mut query_string = pairs.next().unwrap();
        while let Some(pair) = pairs.next() {
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
}
