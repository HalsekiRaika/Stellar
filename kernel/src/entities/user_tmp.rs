use crate::command::TemporalUserCommand;
use crate::entities::{Address, FactorCode, Password, User, UserId, UserName};
use crate::errors::KernelError;
use crate::event::TemporalUserEvent;
use destructure::{Destructure, Mutation};
use error_stack::{Report, ResultExt};
use lutetium::actor::{Actor, ActorContext, Context, Handler};
use lutetium::system::LutetiumActorSystem;

#[derive(Debug, Clone, Destructure, Mutation)]
pub struct TemporalUser {
    id: UserId,
    state: TemporalUserFlowState
}

#[derive(Debug, Clone)]
pub(crate) enum TemporalUserFlowState {
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

#[async_trait::async_trait]
impl Handler<TemporalUserCommand> for TemporalUser {
    type Accept = TemporalUserEvent;
    type Rejection = Report<KernelError>;

    async fn call(&mut self, msg: TemporalUserCommand, ctx: &mut Context) -> Result<Self::Accept, Self::Rejection> {
        let ev = match msg {
            TemporalUserCommand::Verification2FA { code } => {
                let TemporalUserFlowState::AddressChecking { address, mfa } = &self.state else {
                    return Err(Report::new(KernelError::Unavailable {
                        
                    }))
                };
                
                mfa.verify(&code)?;
                
                self.state = TemporalUserFlowState::MFAChecked {
                    verified: address.to_owned(),
                };
                
                TemporalUserEvent::Verified2FA
            }
            TemporalUserCommand::BasicInfoRegistration { name, pass } => {
                let TemporalUserFlowState::MFAChecked { .. } = &self.state else {
                    return Err(Report::new(KernelError::Unavailable {
                        
                    }))
                };
                let name = UserName::new(name);
                let pass = Password::new(pass)?;
                
                let user = User::new(self.id().to_owned(), name, pass);
                
                let _ = ctx.system()
                    .spawn(self.id().to_owned(), user)
                    .await
                    .change_context_lazy(|| KernelError::External)?;

                ctx.shutdown();
                
                TemporalUserEvent::Registration
            }
            _ => {
                return Err(Report::new(KernelError::Unavailable {}))
            }
        };
        Ok(ev)
    }
}