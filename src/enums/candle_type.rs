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
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum CandleType {
    Classic,
    HeikinAshi,
}
