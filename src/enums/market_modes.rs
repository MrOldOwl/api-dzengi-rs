#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketModes {
    ClosedForCorporateAction,
    CloseOnly,
    Delisting,
    Holiday,
    LongOnly,
    Regular,
    Unknown,
    ViewAndRequest,
    ViewOnly,
}
