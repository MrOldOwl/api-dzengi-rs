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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}
