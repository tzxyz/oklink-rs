mod api;
pub mod request;
pub mod response;

pub use api::*;

mod tests {

    use super::request::*;
    use super::response::*;
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

    #[tokio::test]
    async fn test_position_list() {
        setup();
        let request = PositionListRequest {
            chain_short_name: "TRON".into(),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            holder_address: Some("TRqQ15XEJyAsWbTfgNjGWLTBcCr23rM9LW".into()),
            ..Default::default()
        };
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .general()
            .token()
            .position_list(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_transaction_list() {
        setup();
        let request = TransactionListRequest {
            chain_short_name: "TRON".into(),
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".into(),
            ..Default::default()
        };
        let result = OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
            .general()
            .token()
            .transaction_list(request)
            .await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
