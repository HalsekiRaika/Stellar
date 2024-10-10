use crate::command::{TemporalUserCommand, TemporalUserSignup, UserRegistrationCommand};
use crate::entities::{Address, FactorCode, Password, User, UserId, UserName};
use crate::errors::KernelError;
use crate::event::TemporalUserEvent;
use destructure::{Destructure, Mutation};
use error_stack::{Report, ResultExt};
use lutetium::actor::{Actor, ActorContext, Context, Handler, TryIntoActor};
use lutetium::system::LutetiumActorSystem;

#[derive(Debug, Clone, Destructure, Mutation)]
pub struct TemporalUser {
    id: UserId,
    state: TemporalUserFlowState
}

#[derive(Debug, Clone)]
pub(crate) enum TemporalUserFlowState {
    WaitAddressInput,
    AddressChecking { address: Address, mfa: FactorCode },
    MFAChecked { verified: Address }
}

impl TemporalUser {
    pub fn id(&self) -> &UserId {
        &self.id
    }
}


impl Actor for  TemporalUser {
    type Context = Context;
}

impl TryIntoActor<TemporalUser> for TemporalUserSignup {
    type Identifier = UserId;
    type Rejection = Report<KernelError>;

    fn try_into_actor(self, id: UserId) -> Result<(Self::Identifier, TemporalUser), Self::Rejection> {
        Ok((id, TemporalUser { id, state: TemporalUserFlowState::WaitAddressInput }))
    }
}


#[async_trait::async_trait]
impl Handler<TemporalUserCommand> for TemporalUser {
    type Accept = TemporalUserEvent;
    type Rejection = Report<KernelError>;

    #[tracing::instrument(skip_all, name = "TemporalUser", fields(id = %self.id))]
    async fn call(&mut self, msg: TemporalUserCommand, ctx: &mut Context) -> Result<Self::Accept, Self::Rejection> {
        let ev = match msg {
            TemporalUserCommand::EnterAddress { address} => {
                let TemporalUserFlowState::WaitAddressInput = &self.state else {
                    return Err(Report::new(KernelError::Unavailable {
                        
                    }))
                };
                
                let code = FactorCode::generate();
                
                self.state = TemporalUserFlowState::AddressChecking {
                    address: Address::new(address),
                    mfa: code.clone(),
                };
                
                tracing::debug!("wait verifying address");
                
                TemporalUserEvent::AcceptedAddress { code }
            }
            TemporalUserCommand::Verification2FA { code } => {
                let TemporalUserFlowState::AddressChecking { address, mfa } = &self.state else {
                    return Err(Report::new(KernelError::Unavailable {
                        
                    }))
                };
                
                mfa.verify(&code)?;
                
                self.state = TemporalUserFlowState::MFAChecked {
                    verified: address.to_owned(),
                };
                
                tracing::debug!("mfa accepted");
                
                TemporalUserEvent::Verified2FA
            }
            TemporalUserCommand::BasicInfoRegistration { name, pass } => {
                let TemporalUserFlowState::MFAChecked { verified } = &self.state else {
                    return Err(Report::new(KernelError::Unavailable {
                        
                    }))
                };
                
                let id = UserId::default();
                let name = UserName::new(name);
                let pass = Password::new(pass)?;
                
                let cmd = UserRegistrationCommand { 
                    id, 
                    name, 
                    pass, 
                    address: verified.to_owned() 
                };

                ctx.shutdown().await;
                
                let _ = ctx.system()
                    .spawn_from::<User, _>(cmd)
                    .await?
                    .change_context_lazy(|| KernelError::External { crate_name: "lutetium" })?;

                tracing::debug!("user registered");
                
                TemporalUserEvent::Registration { user_id: id }
            }
        };
        Ok(ev)
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use error_stack::{Report, ResultExt};
    use lutetium::actor::refs::RegularAction;
    use lutetium::system::{ActorSystem, LutetiumActorSystem};
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use crate::command::{TemporalUserCommand, TemporalUserSignup};
    use crate::entities::UserId;
    use crate::errors::KernelError;
    use crate::event::TemporalUserEvent;

    #[tokio::test]
    async fn actor_test() -> Result<(), Report<KernelError>> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer()
                      .with_filter(tracing_subscriber::EnvFilter::new("kernel=trace,lutetium=debug"))
                      .with_filter(tracing_subscriber::filter::LevelFilter::TRACE),
            )
            .init();
        
        let system = ActorSystem::builder().build();
        let id = UserId::default();
        let refs = system.try_spawn(id, TemporalUserSignup).await?
            .change_context_lazy(|| KernelError::External { crate_name: "lutetium" })?;

        let ev = refs.ask(TemporalUserCommand::EnterAddress { address: "test@example.com".to_string() }).await
            .change_context_lazy(|| KernelError::External { crate_name: "lutetium" })??;
        
        let TemporalUserEvent::AcceptedAddress { code } = ev else {
            return Err(Report::new(KernelError::Invalid { reason: "decline address" }))
        };
        
        let ev = refs.ask(TemporalUserCommand::Verification2FA { code: code.into() }).await
            .change_context_lazy(|| KernelError::External { crate_name: "lutetium" })??;
        
        let TemporalUserEvent::Verified2FA = ev else {
            return Err(Report::new(KernelError::Invalid { reason: "Failed 2FA" }))
        };

        let ev = refs.ask(TemporalUserCommand::BasicInfoRegistration { name: "test-user".to_string(), pass: "test123".to_string() }).await
            .change_context_lazy(|| KernelError::External { crate_name: "lutetium" })??;
        
        let TemporalUserEvent::Registration { user_id } = ev else {
            return Err(Report::new(KernelError::Invalid { reason: "Failed registration" }))
        };
        
        println!("Generated {}", user_id);
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        Ok(())
    }
}