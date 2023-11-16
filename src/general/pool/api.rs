use crate::generate_api;
use crate::OkLink;

use super::request::*;
use super::response::*;

pub struct PoolModule {
    pub inner: OkLink,
}

impl PoolModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

generate_api!(
    PoolModule,
    estimated_pool_share,
    "/api/v5/explorer/pool/estimated-pool-share",
    EstimatedPoolShareRequest,
    EstimatedPoolShareResponse;

    pool_hashrate_rank,
    "/api/v5/explorer/pool/pool-hashrate-rank",
    PoolHashRateRankRequest,
    PoolHashRateRankResponse;

    validator_list,
    "/api/v5/explorer/pool/validator-list",
    ValidatorListRequest,
    ValidatorListResponse
);

#[cfg(test)]
mod tests {

    use crate::general::pool::request::*;
    use crate::*;

    fn setup() -> OkLink {
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    pub async fn test_estimated_pool_share() {
        let client = setup();
        let request = EstimatedPoolShareRequest {
            chain_short_name: "BTC".into(),
            ..Default::default()
        };
        let result = client.general().pool().estimated_pool_share(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    pub async fn test_pool_hashrate_rank() {
        let client = setup();
        let request = PoolHashRateRankRequest {
            chain_short_name: "BTC".into(),
            ..Default::default()
        };
        let result = client.general().pool().pool_hashrate_rank(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    pub async fn test_validator_list() {
        let client = setup();
        let request = ValidatorListRequest {
            chain_short_name: "TRON".into(),
            ..Default::default()
        };
        let result = client.general().pool().validator_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
