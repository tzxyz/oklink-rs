use crate::response::PageResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSummaryResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub address: String,
    pub contract_address: String,
    pub balance: String,
    pub balance_symbol: String,
    pub transaction_count: String,
    pub verifying: String,
    pub send_amount: String,
    pub receive_amount: String,
    pub token_amount: String,
    pub total_token_value: String,
    pub create_contract_address: String,
    pub create_contract_transaction_hash: String,
    pub first_transaction_time: String,
    pub last_transaction_time: String,
    pub token: String,
    pub bandwidth: String,
    pub energy: String,
    pub voting_rights: String,
    pub unclaimed_voting_rewards: String,
    pub is_aa_address: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationEvmResponse {
    pub address: String,
    pub balance: String,
    pub balance_symbol: String,
    pub transaction_count: String,
    pub first_transaction_time: String,
    pub last_transaction_time: String,
    pub contract_address: bool,
    pub create_contract_address: String,
    pub create_contract_transaction_hash: String,
    pub contract_corresponding_token: String,
    pub contract_calls: String,
    pub contract_calling_addresses: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceInnerResponse {
    pub symbol: String,
    pub token_contract_address: String,
    pub holding_amount: String,
    pub price_usd: String,
    pub value_usd: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub token_list: Vec<TokenBalanceInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBalanceFillsInnerResponse {
    pub token: String,
    pub token_id: String,
    pub holding_amount: String,
    pub total_token_value: String,
    pub change24h: String,
    pub price_usd: String,
    pub value_usd: String,
    pub token_contract_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBalanceFillsResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub token_list: Vec<AddressBalanceFillsInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBalanceHistoryResponse {
    pub address: String,
    pub height: String,
    pub balance: String,
    pub balance_symbol: String,
    pub token_contract_address: String,
    pub block_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListInnerResponse {
    pub tx_id: String,
    pub method_id: String,
    pub block_hash: String,
    pub height: String,
    pub transaction_time: String,
    pub from: String,
    pub to: String,
    pub is_to_contract: bool,
    pub is_from_contract: bool,
    pub amount: String,
    pub transaction_symbol: String,
    pub tx_fee: String,
    pub state: String,
    pub token_id: String,
    pub token_contract_address: String,
    pub challenge_status: String,
    pub l1_origin_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub transaction_list: Vec<TransactionListInnerResponse>,
}
