mod token;

use crate::OkLink;
use token::TokenMoudle;

pub struct GeneralMoudle {
    inner: OkLink,
}

impl GeneralMoudle {
    pub fn new(oklink: OkLink) -> Self {
        Self {
            inner: oklink.clone(),
        }
    }

    pub fn token(self) -> TokenMoudle {
        TokenMoudle::new(self.inner.clone())
    }
}
