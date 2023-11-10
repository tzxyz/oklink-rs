use super::request::*;
use super::response::*;
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
        log::debug!("{}", api_url);
        let result = self
            .inner
            .get_with_query::<TokenListRequest, TokenListResponse>(api_url, request)
            .await?;
        Ok(result)
    }

    pub async fn position_list(self, request: PositionListRequest) -> Result<PositionListResponse> {
        let api_url = format!(
            "{}{}",
            self.inner.base_url, "/api/v5/explorer/token/position-list"
        );
        log::debug!("{}", api_url);
        let result = self
            .inner
            .get_with_query::<PositionListRequest, PositionListResponse>(api_url, request)
            .await?;
        Ok(result)
    }

    pub async fn transaction_list(
        self,
        request: TransactionListRequest,
    ) -> Result<TransactionListResponse> {
        let api_url = format!(
            "{}{}",
            self.inner.base_url, "/api/v5/explorer/token/transaction-list"
        );
        log::debug!("{}", api_url);
        let result = self
            .inner
            .get_with_query::<TransactionListRequest, TransactionListResponse>(api_url, request)
            .await?;
        Ok(result)
    }
}
