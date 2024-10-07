use error_stack::Report;
use lutetium::actor::{Message, TryIntoActor};
use crate::entities::{Password, User, UserId, UserName};
use crate::errors::KernelError;

#[derive(Debug, Clone)]
pub enum UserCommand {
    Register { name: String, pass: String },
}

impl Message for UserCommand {}

impl TryIntoActor<User> for UserCommand {
    type Identifier = UserId;
    type Rejection = Report<KernelError>;

    fn try_into_actor(self, id: Self::Identifier) -> Result<(Self::Identifier, User), Self::Rejection> {
        let Self::Register { name, pass } = self else {
            return Err(Report::new(KernelError::Invalid {
                reason: "data that could have been a starting point was expected, but was not."
            }))
        };
        Ok((id, User::new(id, UserName::new(name), Password::new(pass)?)))
    }
}