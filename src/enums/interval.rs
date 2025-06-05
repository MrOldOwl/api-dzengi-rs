#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        use Interval::*;
        match self {
            OneMinute => "1m",
            FiveMinutes => "5m",
            FifteenMinutes => "15m",
            ThirtyMinutes => "30m",
            OneHour => "1h",
            FourHours => "4h",
            OneDay => "1d",
            OneWeek => "1w",
        }
        .into()
    }
}
