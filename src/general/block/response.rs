use crate::response::PageResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockFillsResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub hash: String,
    pub height: String,
    pub validator: String,
    pub block_time: String,
    pub txn_count: String,
    pub amount: String,
    pub block_size: String,
    pub mine_reward: String,
    pub total_fee: String,
    pub fee_symbol: String,
    pub ommer_block: String,
    pub merkle_root_hash: Option<String>,
    pub gas_used: String,
    pub gas_limit: String,
    pub gas_avg_price: String,
    pub state: String,
    pub burnt: String,
    pub network: Option<String>,
    pub txn_internal: String,
    pub miner: String,
    pub difficuity: String,
    pub nonce: String,
    pub tips: String,
    pub confirm: String,
    pub base_fee_per_gas: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockListInnerResponse {
    pub hash: String,
    pub height: String,
    pub validator: String,
    pub block_time: String,
    pub txn_count: String,
    pub block_size: String,
    pub mine_reward: String,
    pub total_fee: String,
    pub fee_symbol: String,
    pub avg_fee: String,
    pub ommer_block: String,
    pub gas_used: String,
    pub gas_limit: String,
    pub gas_avg_price: String,
    pub state: String,
    pub burnt: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub block_list: Vec<BlockListInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct TransactionListInnerResponse {
    pub txid: String,
    pub method_id: String,
    pub block_hash: String,
    pub height: String,
    pub transaction_time: String,
    pub from: String,
    pub is_from_contract: bool,
    pub is_to_contract: bool,
    pub to: String,
    pub amount: String,
    pub transaction_symbol: String,
    pub txfee: String,
    pub state: String,
    pub token_id: String,
    pub token_contract_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub block_list: Vec<TransactionListInnerResponse>,
}
