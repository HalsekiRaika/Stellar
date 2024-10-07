use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Address {
        Self(address.into())
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.0
    }
}