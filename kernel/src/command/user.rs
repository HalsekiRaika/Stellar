use crate::entities::{Password, UserId, UserName};
use lutetium::actor::Message;

#[derive(Debug, Clone)]
pub struct UserRegistrationCommand {
    pub id: UserId,
    pub name: UserName,
    pub pass: Password,
}

impl Message for UserRegistrationCommand {}

#[derive(Debug, Clone)]
pub enum UserCommand {
}

impl Message for UserCommand {}
