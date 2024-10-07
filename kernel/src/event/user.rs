use lutetium::persistence::errors::{DeserializeError, SerializeError};
use lutetium::persistence::Event;
use serde::{Deserialize, Serialize};
use crate::entities::{Password, UserId, UserName};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserEvent {
    Registered { id: UserId, name: UserName, pass: Password },
}

impl Event for UserEvent {
    const REGISTRY_KEY: &'static str = "user-event";
    
    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bytes)?)
    }
}