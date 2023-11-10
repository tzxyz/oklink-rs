mod address;
mod block;
mod token;

use crate::OkLink;
use address::AddressModule;
use token::TokenModule;

use self::block::BlockModule;

pub struct GeneralMoudle {
    inner: OkLink,
}

impl GeneralMoudle {
    pub fn new(oklink: OkLink) -> Self {
        Self {
            inner: oklink.clone(),
        }
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
