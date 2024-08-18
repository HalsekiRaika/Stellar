mod id;
mod name;
mod pass;

pub use self::{
    id::*,
    name::*,
    pass::*,
};

use destructure::{Destructure, Mutation};
use error_stack::{Report, ResultExt};
use lutetium::actor::{Context, Handler, Prepare};
use lutetium::errors::ActorError;
use lutetium::persistence::actor::PersistenceActor;
use lutetium::persistence::identifier::{PersistenceId, ToPersistenceId};
use lutetium::persistence::RecoverJournal;
use serde::{Deserialize, Serialize};
use crate::command::UserCommand;
use crate::errors::KernelError;
use crate::event::UserEvent;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct User {
    id: UserId,
    name: UserName,
    pass: Password
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
        pass: Password
    ) -> User {
        Self {
            id,
            name,
            pass
        }
    }
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }
    
    pub fn name(&self) -> &UserName {
        &self.name
    }
    
    pub fn pass(&self) -> &Password {
        &self.pass
    }
}

impl PersistenceActor for User {
    fn persistence_id(&self) -> PersistenceId {
        self.id.to_persistence_id()
    }
}

#[async_trait::async_trait]
impl RecoverJournal<UserEvent> for User {
    async fn recover_journal(&mut self, event: UserEvent, _ctx: &mut Context) {
        match event {
            UserEvent::Withdrawal => {
                
            }
            
            _ => unreachable!()
        }
    }
}

#[async_trait::async_trait]
impl Prepare<UserCommand> for User {
    type Identifier = UserId;
    async fn prepare(msg: UserCommand) -> Result<(Self::Identifier, Self), ActorError> {
        if let UserCommand::Register { name, pass } = msg {
            let id = UserId::default();
            let name = UserName::new(name);
            let pass = Password::new(pass)?;
            
            return Ok((id, User::new(id, name, pass)))
        }
        
        Err(ActorError::NotEnoughValue)
    }
}

#[async_trait::async_trait]
impl Handler<UserCommand> for User {
    type Accept = UserEvent;
    type Rejection = Report<KernelError>;
    async fn call(&mut self, msg: UserCommand, ctx: &mut Context) -> Result<Self::Accept, Self::Rejection> {
        let ev = match msg {
            UserCommand::Withdrawal { pass } => {
                self.pass().verify(pass)?;
                UserEvent::Withdrawal
            }
            _ => unreachable!()
        };
        
        self.persist(&ev, ctx).await
            .change_context_lazy(|| KernelError::Processing)?;
        
        Ok(ev)
    }
}
