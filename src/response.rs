use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSuppertedApiResponse {
    pub chain_full_name: String,
    pub chain_short_name: String,
    pub chain_supported_apis: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSupportedChainsResponse {
    pub api_supported_chains: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    pub page: String,
    pub limit: String,
    pub total_page: Option<String>,
    pub chain_full_name: Option<String>,
    pub chain_short_name: Option<String>,
    pub total_transfer: Option<String>,
}
