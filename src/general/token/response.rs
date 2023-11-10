use crate::response::PageResponse;
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
