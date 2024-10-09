use error_stack::Report;
use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};
use crate::errors::KernelError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct FactorCode(String);

impl FactorCode {
    pub fn generate() -> FactorCode {
        let token = Alphanumeric
            .sample_iter(&mut rand::thread_rng())
            .take(8)
            .map(char::from)
            .collect::<String>();
        Self(token)
    }
    
    pub fn verify(&self, input: &str) -> Result<(), Report<KernelError>> {
        if !self.0.eq(input) {
            return Err(Report::new(KernelError::Invalid {
                reason: "mismatch-factor-code",
            }))
        }
        Ok(())
    }
}

impl AsRef<str> for FactorCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<FactorCode> for String {
    fn from(value: FactorCode) -> Self {
        value.0
    }
}