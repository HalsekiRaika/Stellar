use crate::{entities::{VerificationCode, Address}, KernelError};

#[async_trait::async_trait]
pub trait VerificationMailTransporter: 'static + Sync + Send {
    async fn send(&self, code: &VerificationCode, address: &Address) -> Result<(), KernelError>;
}