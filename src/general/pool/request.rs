use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPoolShareRequest {
    pub chain_short_name: String,
    pub period: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolHashRateRankRequest {
    pub chain_short_name: String,
    pub category: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorListRequest {
    pub chain_short_name: String,
    pub period: Option<String>,
    pub validator_name: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
