pub use lutetium::actor::refs::*;

pub use lutetium::persistence::PersistSystemExt;
pub use lutetium::system::LutetiumActorSystem;

pub trait DependOnActorSystem: 'static + Sync + Send {
    type ActorSystem: LutetiumActorSystem;
    type PersistActorSystem: PersistSystemExt;
    fn actor_system(&self) -> Self::ActorSystem;
    fn persist_actor_system(&self) -> Self::PersistActorSystem;
}