use super::request::*;
use super::response::*;
use crate::generate_api;
use crate::OkLink;

pub struct ChainInfoModule {
    pub inner: OkLink,
}

impl ChainInfoModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

generate_api!(
    ChainInfoModule,
    summary,
    "/api/v5/explorer/blockchain/summary",
    SummaryRequest,
    SummaryResponse;

    info,
    "/api/v5/explorer/blockchain/info",
    InfoRequest,
    InfoResponse;

    block,
    "/api/v5/explorer/blockchain/block",
    BlockRequest,
    BlockResponse;

    address,
    "/api/v5/explorer/blockchain/address",
    AddressRequest,
    AddressResponse;

    fee,
    "/api/v5/explorer/blockchain/fee",
    FeeRequest,
    FeeResponse;

    transaction,
    "/api/v5/explorer/blockchain/transaction",
    TransactionRequest,
    TransactionResponse;

    hashes,
    "/api/v5/explorer/blockchain/hashes",
    HashesRequest,
    HashesResponse;

    mine,
    "/api/v5/explorer/blockchain/mine",
    MineRequest,
    MineResponse
);

#[cfg(test)]
mod tests {
    use crate::general::chain_info::request::*;
    use crate::*;

    fn setup() -> OkLink {
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    async fn test_summary() {
        let client = setup();
        let request = SummaryRequest {
            chain_short_name: Some("BTC".into()),
        };
        let result = client.general().chain_info().summary(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_info() {
        let client = setup();
        let request = InfoRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().info(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_block() {
        let client = setup();
        let request = BlockRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().block(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_address() {
        let client = setup();
        let request = AddressRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().address(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_fee() {
        let client = setup();
        let request = FeeRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().fee(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_transaction() {
        let client = setup();
        let request = TransactionRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().transaction(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_hashes() {
        let client = setup();
        let request = HashesRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().hashes(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_mine() {
        let client = setup();
        let request = MineRequest {
            chain_short_name: "BTC".into(),
        };
        let result = client.general().chain_info().mine(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
