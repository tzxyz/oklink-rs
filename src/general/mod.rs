mod address;
mod token;

use crate::OkLink;
use address::AddressModule;
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

    pub fn address(self) -> AddressModule {
        AddressModule::new(self.inner.clone())
    }

    pub fn token(self) -> TokenModule {
        TokenModule::new(self.inner.clone())
    }
}
