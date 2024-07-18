use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new(id: impl Into<Uuid>) -> ClientId {
        Self(id.into())
    }
}

impl From<ClientId> for Uuid {
    fn from(value: ClientId) -> Self {
        value.0
    }
}

impl AsRef<Uuid> for ClientId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}