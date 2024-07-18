use std::collections::hash_set::Iter;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::errors::KernelError;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedirectUris(HashSet<String>);

impl RedirectUris {
    pub fn add(&mut self, uri: String) -> Result<(), KernelError> {
        if self.0.contains(&uri) { 
            return Err(KernelError::AlreadyExist {
                val: uri
            })
        }
        
        self.0.insert(uri);
        Ok(())
    }
    
    pub fn remove(&mut self, uri: String) -> Result<(), KernelError> {
        if !self.0.remove(&uri) { 
            return Err(KernelError::NotFound {
                val: uri,
            })
        }
        Ok(())
    }
    
    pub fn iter(&self) -> Iter<String> {
        self.0.iter()
    }
}

impl From<RedirectUris> for HashSet<String> {
    fn from(value: RedirectUris) -> Self {
        value.0
    }
}