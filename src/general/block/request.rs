use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockFillsRequest {
    pub chain_short_name: String,
    pub height: String,
    pub net_work: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockListRequest {
    pub chain_short_name: String,
    pub height: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListRequest {
    pub chain_short_name: String,
    pub height: String,
    pub protocol_type: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}
