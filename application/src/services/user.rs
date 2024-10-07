use error_stack::{Report, ResultExt};
use kernel::command::UserCommand;
use kernel::entities::UserId;
use kernel::event::UserEvent;
use kernel::services::{DependOnActorSystem, LutetiumActorSystem, PersistSystemExt, RegularAction};
use crate::errors::ApplicationError;

#[async_trait::async_trait(?Send)]
pub trait UserCommandExecutorService:
    'static
    + Sync
    + Send
    + DependOnActorSystem
{
    async fn execute(&self, id: impl Into<Option<UserId>>, cmd: UserCommand) -> Result<UserEvent, Report<ApplicationError>> {
        let refs = match cmd {
            UserCommand::Register { .. } => {
                let id = UserId::default();
                self.persist_actor_system()
                    .try_spawn(id, cmd.clone())
                    .await
                    .change_context_lazy(|| ApplicationError::Kernel)?
                    .change_context_lazy(|| ApplicationError::External)?
            }
            _ => {
                let id = id.into().ok_or(ApplicationError::Require {
                    require: "user_id"
                })?;
                
                self.persist_actor_system()
                    .find_or_spawn_with_recovery(id, |_| async move { None })
                    .await
                    .change_context_lazy(|| ApplicationError::External)?
            }
        };
        
        let ev = refs.ask(cmd).await
            .change_context_lazy(|| ApplicationError::External)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        Ok(ev)
    }
}
