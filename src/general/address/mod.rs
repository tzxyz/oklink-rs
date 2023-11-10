mod api;
pub mod request;
pub mod response;
pub use api::*;

use crate::generate_api;
use request::*;
use response::*;

generate_api!(
    AddressModule,
    address_summary,
    "/api/v5/explorer/address/address-summary",
    AddressSummaryRequest,
    AddressSummaryResponse;

    information_evm,
    "/api/v5/explorer/address/information-evm",
    InformationEvmRequest,
    InformationEvmResponse
);
