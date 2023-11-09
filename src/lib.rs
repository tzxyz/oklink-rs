mod response;

use std::fmt::Debug;

use reqwest;
use response::{ApiResponse, ChainSuppertedApiResponse};
use serde::de::DeserializeOwned;

pub struct OkLink {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

#[derive(Default)]
pub struct OkLinkBuilder {
    client: Option<reqwest::Client>,
    api_key: Option<String>,
    base_url: Option<String>,
}

impl OkLinkBuilder {
    pub fn new_with_api_key(self, api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..self
        }
    }

    pub fn new_with_base_url(self, base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..self
        }
    }

    pub fn new_with_client(self, client: reqwest::Client) -> Self {
        Self {
            client: Some(client),
            ..self
        }
    }

    pub fn build(self) -> OkLink {
        OkLink {
            client: self.client.unwrap_or_default(),
            api_key: self.api_key.unwrap(),
            base_url: self
                .base_url
                .unwrap_or("https://www.oklink.com".to_string()),
        }
    }
}

impl OkLink {
    pub fn builder() -> OkLinkBuilder {
        Default::default()
    }

    pub fn new(api_key: impl Into<String>) -> Self {
        OkLinkBuilder::default().new_with_api_key(api_key).build()
    }

    pub async fn get<R: Debug + DeserializeOwned>(
        self,
        api_url: String,
    ) -> Result<ApiResponse<R>, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get(api_url)
            .header("Ok-Access-Key", self.api_key)
            .send()
            .await?
            .json::<ApiResponse<R>>()
            .await?;
        println!("{:?}", result);
        Ok(result)
    }

    pub async fn chain_supperted_api(
        self,
        chain_short_name: impl Into<String>,
    ) -> Result<ApiResponse<ChainSuppertedApiResponse>, Box<dyn std::error::Error>> {
        let api_url = format!(
            "{}{}{}",
            self.base_url,
            "/api/v5/explorer/chain-supported-apis?chainShortName=",
            chain_short_name.into()
        );
        println!("{}", api_url);
        let result = self.get::<ChainSuppertedApiResponse>(api_url).await?;
        Ok(result)
    }
}

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_chain_supperted_api() {
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .chain_supperted_api("TRON")
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}