use error_stack::{Report, ResultExt};
use kernel::command::UserCommand;
use kernel::entities::UserId;
use kernel::event::UserEvent;
use kernel::services::{DependOnActorSystem, LutetiumActorSystem, PersistSystemExt, RegularAction};
use crate::errors::ApplicationError;

#[async_trait::async_trait]
pub trait UserCommandExecutorService:
    'static
    + Sync
    + Send
    + DependOnActorSystem
{
    async fn execute(&self, id: Option<UserId>, cmd: UserCommand) -> Result<UserEvent, Report<ApplicationError>> {
        todo!()
    }
}
