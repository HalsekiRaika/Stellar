mod id;
mod name;
mod pass;

pub use self::{
    id::*,
    name::*,
    pass::*,
};

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

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

