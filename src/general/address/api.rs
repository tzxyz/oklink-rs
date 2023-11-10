use super::request::*;
use super::response::*;
use crate::generate_api;
use crate::OkLink;

pub struct AddressModule {
    pub inner: OkLink,
}

impl AddressModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

generate_api!(
    AddressModule,
    address_summary,
    "/api/v5/explorer/address/address-summary",
    AddressSummaryRequest,
    AddressSummaryResponse;

    information_evm,
    "/api/v5/explorer/address/information-evm",
    InformationEvmRequest,
    InformationEvmResponse;

    token_balance,
    "/api/v5/explorer/address/token-balance",
    TokenBalanceRequest,
    TokenBalanceResponse;

    address_balance_fills,
    "/api/v5/explorer/address/address-balance-fills",
    AddressBalanceFillsRequest,
    AddressBalanceFillsResponse;

    address_balance_history,
    "/api/v5/explorer/block/address-balance-history",
    AddressBalanceHistoryRequest,
    AddressBalanceHistoryResponse;

    transaction_list,
    "/api/v5/explorer/address/transaction-list",
    TransactionListRequest,
    TransactionListResponse
);

#[cfg(test)]
mod tests {

    use crate::general::address::request::*;
    use crate::*;

    fn setup() -> OkLink {
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    async fn test_address_summary() {
        let client = setup();
        let request = AddressSummaryRequest {
            chain_short_name: "TRON".into(),
            address: "TRqQ15XEJyAsWbTfgNjGWLTBcCr23rM9LW".into(),
        };
        let result = client.general().address().address_summary(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_information_evm() {
        let client = setup();
        let request = InformationEvmRequest {
            chain_short_name: "ETH".into(),
            address: "0x1f9090aae28b8a3dceadf281b0f12828e676c326".into(),
        };
        let result = client.general().address().information_evm(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_token_balance() {
        let client = setup();
        let request = TokenBalanceRequest {
            chain_short_name: "ETH".into(),
            address: "0x1f9090aae28b8a3dceadf281b0f12828e676c326".into(),
            protocol_type: "token_20".into(),
            token_contract_address: Some("0xdAC17F958D2ee523a2206206994597C13D831ec7".into()),
            ..Default::default()
        };
        let result = client.general().address().token_balance(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_address_balance_fills() {
        let client = setup();
        let request = AddressBalanceFillsRequest {
            chain_short_name: "ETH".into(),
            address: "0x1f9090aae28b8a3dceadf281b0f12828e676c326".into(),
            ..Default::default()
        };
        let result = client
            .general()
            .address()
            .address_balance_fills(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_address_balance_history() {
        let client = setup();
        let request = AddressBalanceHistoryRequest {
            chain_short_name: "ETH".into(),
            height: 18376634.to_string(),
            address: "0x1f9090aae28b8a3dceadf281b0f12828e676c326".into(),
            ..Default::default()
        };
        let result = client
            .general()
            .address()
            .address_balance_history(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
