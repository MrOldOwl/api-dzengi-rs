pub struct DefaultKeys;

impl DefaultKeys {
    pub fn timestamp() -> &'static str {
        "timestamp"
    }

    pub fn api_key() -> &'static str {
        "X-MBX-APIKEY"
    }

    pub fn signature() -> &'static str {
        "signature"
    }
}
