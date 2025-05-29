use super::FundingLimitsDtoResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingLimitsDtoResponseWS {
    pub funding_limits: Vec<FundingLimitsDtoResponse>,
}
