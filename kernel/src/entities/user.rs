mod id;
mod name;
mod pass;
mod address;
mod factor_code;

pub use self::{
    id::*,
    name::*,
    pass::*,
    address::*,
    factor_code::*,
};

use destructure::{Destructure, Mutation};
use error_stack::{Report, ResultExt};
use lutetium::actor::Handler;
use lutetium::persistence::actor::PersistenceActor;
use lutetium::persistence::errors::{DeserializeError, SerializeError};
use lutetium::persistence::identifier::{PersistenceId, ToPersistenceId, Version};
use lutetium::persistence::{PersistContext, RecoverJournal, RecoverSnapShot, SnapShot};
use lutetium::persistence::mapping::{RecoverMapping, RecoveryMapping};
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
    const VERSION: Version = Version::new("0.1.0");

    fn persistence_id(&self) -> PersistenceId {
        self.id.to_persistence_id()
    }
}

impl SnapShot for User {
    const REGISTRY_KEY: &'static str = "user-snapshot";

    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }

    fn from_bytes(bin: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bin)?)
    }
}

#[async_trait::async_trait]
impl RecoverSnapShot for User {
    async fn recover_snapshot(this: &mut Option<Self>, snapshot: Self, _ctx: &mut PersistContext) {
        match this {
            None => {
                *this = Some(Self::new(snapshot.id, snapshot.name, snapshot.pass));
            }
            Some(this) => {
                this.name = snapshot.name;
                this.pass = snapshot.pass;
            }
        }
    }
}

#[async_trait::async_trait]
impl RecoverJournal<UserEvent> for User {
    async fn recover_journal(this: &mut Option<Self>, event: UserEvent, _ctx: &mut PersistContext) {
        match this {
            None => {
                if let UserEvent::Registered { id, name, pass } = event {
                    *this = Some(Self::new(id, name, pass))
                }
            }
            Some(_this) => {
            }
        }
    }
}

impl RecoveryMapping for User {
    fn mapping(mapping: &mut RecoverMapping<Self>) {
        mapping
            .reg_snapshot::<Self>()
            .reg_event::<UserEvent>();
    }
}

#[async_trait::async_trait]
impl Handler<UserCommand> for User {
    type Accept = UserEvent;
    type Rejection = Report<KernelError>;

    async fn call(&mut self, msg: UserCommand, ctx: &mut PersistContext) -> Result<Self::Accept, Self::Rejection> {
        let ev = match msg {
            UserCommand::Register { .. } => {
                self.snapshot(self, ctx).await
                    .change_context_lazy(|| KernelError::External)?;
                UserEvent::Registered {
                    id: self.id,
                    name: self.name.clone(),
                    pass: self.pass.clone(),
                }
            }
        };
        
        self.persist(&ev, ctx).await
            .change_context_lazy(|| KernelError::External)?;
        
        Ok(ev)
    }
}