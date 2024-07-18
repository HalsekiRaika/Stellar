mod id;
mod name;

pub use self::{
    id::*,
    name::*,
};

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
    ) -> User {
        Self {
            id,
            name
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
}