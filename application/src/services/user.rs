use error_stack::{Report, ResultExt};
use kernel::command::UserCommand;
use kernel::entities::UserId;
use kernel::event::UserEvent;
use kernel::services::{DependOnActorSystem, DependOnMailService, LutetiumActorSystem, PersistSystemExt, RegularAction};
use crate::errors::ApplicationError;

#[async_trait::async_trait]
pub trait UserSignupService:
    'static
    + Sync
    + Send
    + DependOnActorSystem
    + DependOnMailService
{
    async fn execute(&self, id: Option<UserId>, cmd: UserCommand) -> Result<UserEvent, Report<ApplicationError>> {
        todo!()
    }
}
