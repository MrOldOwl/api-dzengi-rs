#[derive(
    Debug, Default, Hash, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub enum CorrectionLocalTime {
    #[default]
    None,
    Add(u128),
    Sub(u128),
}
