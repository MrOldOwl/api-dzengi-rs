use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct KlinesResponse {
    #[serde(rename = "0")]
    pub open_time: u128,
    #[serde(rename = "1", deserialize_with = "parse_f64")]
    pub open: f64,
    #[serde(rename = "2", deserialize_with = "parse_f64")]
    pub hight: f64,
    #[serde(rename = "3", deserialize_with = "parse_f64")]
    pub low: f64,
    #[serde(rename = "4", deserialize_with = "parse_f64")]
    pub close: f64,
    #[serde(rename = "5")]
    pub volume: f64,
}

fn parse_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().map_err(|_| D::Error::custom("bad price parse"))?)
}
