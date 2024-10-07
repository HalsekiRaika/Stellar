use error_stack::Report;
use crate::entities::Address;
use crate::errors::KernelError;

#[async_trait::async_trait]
pub trait MailService: 'static + Sync + Send {
    async fn send(&self, address: &Address) -> Result<(), Report<KernelError>>;
}

pub trait DependOnMailService: 'static + Sync + Send {
    type MailService: MailService;
    fn mail_service(&self) -> Self::MailService;
}
