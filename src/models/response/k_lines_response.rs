#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KLinesResponse {
    pub lines: Vec<Vec<()>>,
}
