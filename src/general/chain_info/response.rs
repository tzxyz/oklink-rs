use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryResponse {
    chain_full_name: String,
    chain_short_name: String,
    symbol: String,
    last_height: String,
    last_block_time: String,
    circulating_supply: String,
    circulating_supply_proportion: String,
    transactions: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub rank: String,
    pub mineable: bool,
    pub algorithm: String,
    pub consensus: String,
    pub diff_estimation: String,
    pub current_diff: String,
    pub diff_adjust_time: String,
    pub circulating_supply: String,
    pub total_supply: String,
    pub tps: String,
    pub last_height: String,
    pub last_block_time: String,
    pub issue_date: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub last_height: String,
    pub first_exchange_historical_time: String,
    pub first_block_time: String,
    pub first_block_height: String,
    pub avg_block_interval: String,
    pub avg_block_size_24h: String,
    pub avg_block_size_24h_percent: String,
    pub media_block_size: String,
    pub halve_time: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub valid_address_count: String,
    pub new_address_count_24h: String,
    pub total_addresses: String,
    pub new_total_addresses_24h: String,
    pub contract_addresses: String,
    pub new_contract_addresses_24h: String,
    pub external_addresses: String,
    pub new_external_addresses_24h: String,
    pub active_addresses: String,
    pub new_active_addresses: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub best_transaction_fee: String,
    pub best_transaction_fee_sat: String,
    pub recommended_gas_price: String,
    pub rapid_gas_price: String,
    pub standard_gas_price: String,
    pub slow_gas_price: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub pending_transaction_count: String,
    pub transaction_value_24h: String,
    pub total_transaction_count: String,
    pub tran_rate: String,
    pub avg_transaction_count_24h: String,
    pub avg_transaction_count_24h_percent: String,
    pub pending_transaction_size: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashesResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub hash_rate: String,
    pub hash_rate_change_24h: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MineResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub symbol: String,
    pub avg_mine_reward_24h: String,
    pub miner_income_per_unit: String,
    pub miner_income_per_unit_coin: String,
}
