use crate::response::PageResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPoolShareResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub rank: String,
    pub pool_name: String,
    pub hashrate: String,
    pub ratio: String,
    pub block_count: String,
    pub empty_block_count: String,
    pub ommer_block_count: String,
    pub avg_block_size: String,
    pub avg_fee: String,
    pub miner_fee_ratio: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolHashRateRankResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub rank: String,
    pub pool_name: String,
    pub hashrate: String,
    pub change_24h: String,
    pub lucky_ratio: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorListInnerResponse {
    pub rank: String,
    pub validator_name: String,
    pub validator_address: String,
    pub weight_ratio: String,
    pub weight: String,
    pub blocks: String,
    pub staked: String,
    pub staked_symbol: String,
    pub reward: f64,
    pub reward_symbol: String,
    pub state: String,
    pub first_height: String,
    pub latest_height: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub validator_list: Vec<ValidatorListInnerResponse>,
}
