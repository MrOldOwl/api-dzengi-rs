#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumCount,
    strum_macros::EnumString,
    strum_macros::EnumIs,
    strum_macros::VariantArray,
    strum_macros::VariantNames,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Interval {
    #[serde(rename = "1m")]
    #[strum(serialize = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    #[strum(serialize = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    #[strum(serialize = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    #[strum(serialize = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    #[strum(serialize = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    #[strum(serialize = "1w")]
    OneWeek,
}
