use super::request::*;
use super::response::*;
use crate::generate_api;
use crate::OkLink;

pub struct BlockModule {
    pub inner: OkLink,
}

impl BlockModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

generate_api!(
    BlockModule,
    block_fills,
    "/api/v5/explorer/block/block-fills",
    BlockFillsRequest,
    BlockFillsResponse;

    block_list,
    "/api/v5/explorer/block/block-list",
    BlockListRequest,
    BlockListResponse;

    transaction_list,
    "/api/v5/explorer/block/transaction-list",
    TransactionListRequest,
    TransactionListResponse
);

#[cfg(test)]
mod tests {

    use crate::general::block::request::*;
    use crate::*;

    fn setup() -> OkLink {
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    async fn test_block_fills() {
        let client = setup();
        let request = BlockFillsRequest {
            chain_short_name: "ETH".into(),
            height: "10000".to_string(),
            ..Default::default()
        };
        let result = client.general().block().block_fills(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_block_list() {
        let client = setup();
        let request = BlockListRequest {
            chain_short_name: "ETH".into(),
            height: Some(10000.to_string()),
            ..Default::default()
        };
        let result = client.general().block().block_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_transaction_list() {
        let client = setup();
        let request = TransactionListRequest {
            chain_short_name: "ETH".into(),
            height: 10000.to_string(),
            ..Default::default()
        };
        let result = client.general().block().transaction_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
