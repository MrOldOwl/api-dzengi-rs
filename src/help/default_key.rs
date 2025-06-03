pub struct DefaultKeys;

impl DefaultKeys {
    pub fn symbol() -> &'static str {
        "symbol"
    }

    pub fn timestamp() -> &'static str {
        "timestamp"
    }

    pub fn recv_window() -> &'static str {
        "recvWindow"
    }

    pub fn api_key() -> &'static str {
        "X-MBX-APIKEY"
    }

    pub fn signature() -> &'static str {
        "signature"
    }
}
