use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummaryRequest {
    pub chain_short_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonRequest {
    pub chain_short_name: String,
}

pub type InfoRequest = CommonRequest;
pub type BlockRequest = CommonRequest;
pub type AddressRequest = CommonRequest;
pub type FeeRequest = CommonRequest;
pub type TransactionRequest = CommonRequest;
pub type HashesRequest = CommonRequest;
pub type MineRequest = CommonRequest;
