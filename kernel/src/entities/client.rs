mod id;
mod redirects;
mod scopes;

pub use self::{
    id::*,
    redirects::*,
};

use destructure::Destructure;
use serde::{Deserialize, Serialize};
use crate::entities::client::scopes::Scopes;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Client {
    id: ClientId,
    scopes: Scopes,
    redirect_uris: RedirectUris
}

impl Client {
    pub fn new(
        id: ClientId,
        scopes: Scopes,
        redirect_uris: RedirectUris
    ) -> Client {
        Self {
            id,
            scopes,
            redirect_uris,
        }
    }
}

impl Client {
    pub fn id(&self) -> &ClientId {
        &self.id
    }
    
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }
    
    pub fn redirect_uris(&self) -> &RedirectUris {
        &self.redirect_uris
    }
}