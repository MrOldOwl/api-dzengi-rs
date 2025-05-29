#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    Bond,
    Commodity,
    Credit,
    Cryptocurrency,
    Currency,
    Equity,
    Ico,
    Index,
    InterestRate,
    OptTokens,
    OtherAsset,
    RealEstate,
    UtilityTokens,
}
