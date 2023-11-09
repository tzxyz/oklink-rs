use serde::Deserialize;
use serde::Serialize;

use crate::response::PageResponse;
use crate::OkLink;
use crate::Result;

pub struct TokenMoudle {
    inner: OkLink,
}

impl TokenMoudle {
    pub fn new(inner: OkLink) -> Self {
        Self { inner: inner }
    }

    pub async fn token_list(self, request: TokenListRequest) -> Result<TokenListResponse> {
        let api_url = format!(
            "{}{}",
            self.inner.base_url, "/api/v5/explorer/token/token-list"
        );
        println!("{}", api_url);
        let result = self
            .inner
            .get_with_query::<TokenListRequest, TokenListResponse>(api_url, request)
            .await?;
        Ok(result)
    }
}

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

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListRequest {
    pub chain_short_name: String,
    pub protocol_type: Option<String>,
    pub token_contract_address: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

mod tests {

    use super::*;
    use crate::*;
    use tracing_subscriber;

    fn setup() {
        tracing_subscriber::fmt().init();
    }

    #[tokio::test]
    async fn test_token_list() {
        setup();
        let request = TokenListRequest {
            chain_short_name: "TRON".into(),
            ..Default::default()
        };
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .general()
            .token()
            .token_list(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
