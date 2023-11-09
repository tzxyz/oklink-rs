use crate::OkLink;

use self::token::TokenMoudle;

mod token;

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
