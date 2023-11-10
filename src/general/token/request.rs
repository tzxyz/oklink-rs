use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListRequest {
    pub chain_short_name: String,
    pub protocol_type: Option<String>,
    pub token_contract_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionListRequest {
    pub chain_short_name: String,
    pub token_contract_address: String,
    pub holder_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}
