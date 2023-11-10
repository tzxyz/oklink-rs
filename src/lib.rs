mod general;
mod response;

use std::fmt::Debug;

use general::GeneralMoudle;
use reqwest;
use response::{ApiResponse, ApiSupportedChainsResponse, ChainSuppertedApiResponse};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
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

pub type Result<R> = std::result::Result<ApiResponse<R>, Box<dyn std::error::Error>>;

impl OkLink {
    pub fn builder() -> OkLinkBuilder {
        Default::default()
    }

    pub fn new(api_key: impl Into<String>) -> Self {
        OkLinkBuilder::default().new_with_api_key(api_key).build()
    }

    pub async fn get<R: Debug + DeserializeOwned>(self, api_url: String) -> Result<R> {
        let result = self
            .client
            .get(api_url)
            .header("Ok-Access-Key", self.api_key)
            .send()
            .await?
            .json::<ApiResponse<R>>()
            .await?;
        log::info!("{:?}", result);
        Ok(result)
    }

    pub async fn get_with_query<Q: Serialize, R: Debug + DeserializeOwned>(
        self,
        api_url: String,
        query: Q,
    ) -> Result<R> {
        let result = self
            .client
            .get(api_url)
            .header("Ok-Access-Key", self.api_key)
            .query(&query)
            .send()
            .await?
            .json::<ApiResponse<R>>()
            .await?;
        log::info!("{:?}", result);
        Ok(result)
    }

    pub fn general(self) -> GeneralMoudle {
        GeneralMoudle::new(self)
    }

    pub async fn chain_supported_api(
        self,
        chain_short_name: impl Into<String>,
    ) -> Result<ChainSuppertedApiResponse> {
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

    pub async fn api_supported_chains(
        self,
        url: impl Into<String>,
    ) -> Result<ApiSupportedChainsResponse> {
        let api_url = format!(
            "{}{}{}",
            self.base_url,
            "/api/v5/explorer/api-supported-chains?apiUrl=",
            url.into()
        );
        println!("{}", api_url);
        let result = self.get::<ApiSupportedChainsResponse>(api_url).await?;
        Ok(result)
    }
}

#[macro_export]
macro_rules! generate_api {
    ($struct_name:ident, $($method_name:ident, $api_url:expr, $request_type:ty, $response_type:ty);*) => {
        $(
            impl $struct_name {
                pub async fn $method_name(self, request: $request_type) -> crate::Result<$response_type> {
                    let api_url = format!("{}{}", self.inner.base_url, $api_url);
                    log::debug!("{}", api_url);
                    let result = self
                        .inner
                        .get_with_query::<$request_type, $response_type>(api_url, request)
                        .await?;
                    Ok(result)
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_chain_supported_api() {
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .chain_supported_api("TRON")
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_api_supported_chains() {
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .api_supported_chains("/api/v5/explorer/blockchain/info")
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
