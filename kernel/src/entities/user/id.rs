use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
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

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId({})", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}