use lutetium::identifier::{ActorId, IntoActorId, ToActorId};
use lutetium::persistence::identifier::{PersistenceId, ToPersistenceId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl IntoActorId for UserId {
    fn into_actor_id(self) -> ActorId {
        self.0.to_actor_id()
    }
}

impl ToPersistenceId for UserId {
    fn to_persistence_id(&self) -> PersistenceId {
        self.0.to_persistence_id()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}