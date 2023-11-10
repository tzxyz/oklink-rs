use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSummaryRequest {
    pub chain_short_name: String,
    pub address: String,
}

pub type InformationEvmRequest = AddressSummaryRequest;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceRequest {
    pub chain_short_name: String,
    pub address: String,
    pub protocol_type: String,
    pub token_contract_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBalanceFillsRequest {
    pub chain_short_name: String,
    pub address: String,
    pub protocol_type: Option<String>,
    pub token_contract_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBalanceHistoryRequest {
    pub chain_short_name: String,
    pub height: String,
    pub address: String,
    pub token_contract_address: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListRequest {
    pub chain_short_name: String,
    pub address: String,
    pub protocol_type: Option<String>,
    pub token_contract_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}
