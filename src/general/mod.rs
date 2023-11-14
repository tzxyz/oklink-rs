pub mod address;
pub mod block;
pub mod chain_info;
pub mod token;

use crate::OkLink;
use address::AddressModule;
use block::BlockModule;
use chain_info::ChainInfoModule;
use token::TokenModule;

pub struct GeneralMoudle {
    inner: OkLink,
}

impl GeneralMoudle {
    pub fn new(oklink: OkLink) -> Self {
        Self {
            inner: oklink.clone(),
        }
    }

    pub fn chain_info(self) -> ChainInfoModule {
        ChainInfoModule::new(self.inner.clone())
    }

    pub fn block(self) -> BlockModule {
        BlockModule::new(self.inner.clone())
    }

    pub fn address(self) -> AddressModule {
        AddressModule::new(self.inner.clone())
    }

    pub fn token(self) -> TokenModule {
        TokenModule::new(self.inner.clone())
    }
}
