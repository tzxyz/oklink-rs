use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInfoResponse {
    expiration_time: String,
    total_calls: String,
    used_calls: String,
    remained_calls: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHistoryResponse {
    pub time: String,
    pub calls: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopCallsResponse {
    pub rank: String,
    pub api_url: String,
    pub calls: String,
}
