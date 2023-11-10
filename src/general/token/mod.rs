mod api;
pub mod request;
pub mod response;

pub use api::*;

mod tests {

    use super::request::*;
    use crate::*;
    use tracing_subscriber;

    fn setup() -> OkLink {
        tracing_subscriber::fmt().init();
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    async fn test_token_list() {
        let client = setup();
        let request = TokenListRequest {
            chain_short_name: "TRON".into(),
            ..Default::default()
        };
        let result = client.general().token().token_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_position_list() {
        let client = setup();
        let request = PositionListRequest {
            chain_short_name: "TRON".into(),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            holder_address: Some("TRqQ15XEJyAsWbTfgNjGWLTBcCr23rM9LW".into()),
            ..Default::default()
        };
        let result = client.general().token().position_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_transaction_list() {
        let client = setup();
        let request = TransactionListRequest {
            chain_short_name: "TRON".into(),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            ..Default::default()
        };
        let result = client.general().token().transaction_list(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_price_multi() {
        let client = setup();
        let request = PriceMultiRequest {
            chain_short_name: Some("TRON".into()),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            ..Default::default()
        };
        let result = client.general().token().price_multi(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_token_transaction_list_multi() {
        let client = setup();
        let request = TokenTransactionListMultiRequest {
            chain_short_name: "TRON".into(),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            start_block_height: 56322000.to_string(),
            end_block_height: 56322911.to_string(),
            ..Default::default()
        };
        let result = client
            .general()
            .token()
            .token_transaction_list_multi(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
