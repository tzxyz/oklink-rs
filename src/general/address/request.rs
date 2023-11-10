use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSummaryRequest {
    pub chain_short_name: String,
    pub address: String,
}

pub type InformationEvmRequest = AddressSummaryRequest;
