use crate::response::{ApiBaseResponse, PageResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListInnerResponse {
    pub token_full_name: String,
    pub token: String,
    pub precision: String,
    pub token_contract_address: String,
    pub protocol_type: String,
    pub address_count: String,
    pub total_supply: String,
    pub circulating_supply: String,
    pub price: String,
    pub website: String,
    pub total_market_cap: String,
    pub issue_date: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub token_list: Vec<TokenListInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionListInnerResponse {
    pub holder_address: String,
    pub amount: String,
    pub value_usd: String,
    pub position_change24h: String,
    pub rank: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub position_list: Vec<PositionListInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListInnerResponse {
    pub txid: String,
    pub block_hash: String,
    pub height: String,
    pub transaction_time: String,
    pub from: String,
    pub to: String,
    pub is_to_contract: bool,
    pub is_from_contract: bool,
    pub amount: String,
    pub transaction_symbol: String,
    pub method_id: String,
    pub token_contract_address: String,
    pub protocol_type: String,
    pub state: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub transaction_list: Vec<TransactionListInnerResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMultiResponse {
    pub last_price: String,
    pub token_contract_address: String,
    pub price_abnormal: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransactionListMultiInnerResponse {
    pub tx_id: String,
    pub block_hash: String,
    pub height: String,
    pub transaction_time: String,
    pub from: String,
    pub to: String,
    pub is_to_contract: bool,
    pub is_from_contract: bool,
    pub amount: String,
    pub token_contract_address: String,
    pub state: Option<String>,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransactionListMultiResponse {
    #[serde(flatten)]
    pub page: PageResponse,
    pub transaction_list: Vec<TokenTransactionListMultiInnerResponse>,
}
