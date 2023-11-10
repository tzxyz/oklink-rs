use crate::OkLink;

pub struct AddressModule {
    pub inner: OkLink,
}

impl AddressModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

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
}
