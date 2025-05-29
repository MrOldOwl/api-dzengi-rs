#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionSource {
    CloseOut,
    Dealer,
    Sl,
    System,
    Tp,
    User,
}
