use crate::entities::{Address, Password, UserId, UserName};
use lutetium::actor::Message;

#[derive(Debug, Clone)]
pub struct UserRegistrationCommand {
    pub id: UserId,
    pub name: UserName,
    pub pass: Password,
    pub address: Address,
}

impl Message for UserRegistrationCommand {}

#[derive(Debug, Clone)]
pub enum UserCommand {
}

impl Message for UserCommand {}
