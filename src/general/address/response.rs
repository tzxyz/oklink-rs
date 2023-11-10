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
