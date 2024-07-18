use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use crate::errors::KernelError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ScopeMethod(String);

impl ScopeMethod {
    pub fn new(method: impl Into<String>) -> ScopeMethod {
        Self(method.into())
    }
}

impl Display for ScopeMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, Deserialize, Serialize)]
pub struct Scope {
    method: ScopeMethod,
    description: String
}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.method.hash(state);
    }
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.method.eq(&other.method)
    }
}

impl PartialEq<ScopeMethod> for Scope {
    fn eq(&self, other: &ScopeMethod) -> bool {
        self.method.eq(other)
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.method.0)
    }
}


#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scopes(HashSet<Scope>);

impl Scopes {
    pub fn add(&mut self, scope: Scope) -> Result<(), KernelError> {
        if let Some(scope) = self.0.iter().find(|scope| scope.eq(&&scope.method)) {
            return Err(KernelError::AlreadyExist {
                val: scope.to_string(),
            })
        }
        
        self.0.insert(scope);
        
        Ok(())
    }
    
    pub fn remove(&mut self, method: &ScopeMethod) -> Result<(), KernelError> {
        if !self.0.iter().any(|scope| scope.eq(method)) {
            return Err(KernelError::NotFound {
                val: method.to_string(),
            })
        }
        self.0.retain(|scope| scope.eq(method));
        Ok(())
    }
}

impl From<Scopes> for HashSet<Scope> {
    fn from(value: Scopes) -> Self {
        value.0
    }
}