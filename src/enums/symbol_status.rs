#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    AuctionMatch,
    Break,
    EndOfDay,
    Halt,
    PostTrading,
    PreTrading,
    Trading,
}
