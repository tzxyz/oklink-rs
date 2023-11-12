use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHistoryRequest {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}
