#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LeverageSettingsResponse {
    pub value: i32,
    pub values: Vec<i32>,
}
