use crate::generate_api;
use crate::OkLink;

use super::request::*;
use super::response::*;

pub struct ManagementModule {
    inner: OkLink,
}

impl ManagementModule {
    pub fn new(oklink: OkLink) -> Self {
        Self { inner: oklink }
    }
}

generate_api!(
    ManagementModule,
    subscription_info,
    "/api/v5/explorer/management/subscription-info",
    (),
    SubscriptionInfoResponse;

    top_calls,
    "/api/v5/explorer/management/top-calls",
    (),
    TopCallsResponse
);

generate_api!(
    ManagementModule,
    call_history,
    "/api/v5/explorer/management/call-history",
    CallHistoryRequest,
    CallHistoryResponse
);

#[cfg(test)]
mod tests {

    use crate::management::request::*;
    use crate::*;

    fn setup() -> OkLink {
        OkLink::new("75b3c8ce-8270-4f2f-99c0-aca94106a215")
    }

    #[tokio::test]
    pub async fn test_subscription_info() {
        let client = setup();
        let result = client.management().subscription_info().await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    pub async fn top_calls() {
        let client = setup();
        let result = client.management().top_calls().await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    pub async fn test_call_history() {
        let client = setup();
        let request = CallHistoryRequest {
            ..Default::default()
        };
        let result = client.management().call_history(request).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => panic!("{:?}", e),
        }
    }
}
