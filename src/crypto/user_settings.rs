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
    pub fn api_header(&self) -> &'static str {
        "X-MBX-APIKEY"
    }

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
        params: &mut [(impl AsRef<str>, impl AsRef<str>)],
    ) -> Result<Zeroizing<String>, CryptoError> {
        if params.len() == 0 {
            return Err(CryptoError::ParamsEmpty);
        }
        params.sort_by(|lhs, rhs| lhs.0.as_ref().cmp(rhs.0.as_ref()));

        let mut pairs = params
            .iter()
            .map(|x| format!("{}={}", x.0.as_ref(), x.1.as_ref()));

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
