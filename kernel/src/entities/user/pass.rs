use std::fmt::{Display, Formatter};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use error_stack::{Report, ResultExt};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::errors::KernelError;

static HASHER: Lazy<Argon2> = Lazy::new(|| {
    Argon2::default()
});

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(pass: impl Into<String>) -> Result<Password, Report<KernelError>> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = HASHER.hash_password(pass.into().as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .change_context_lazy(|| KernelError::Processing)?;
        Ok(Self(hash))
    }
    
    pub fn verify(&self, pass: impl AsRef<str>) -> Result<(), Report<KernelError>> {
        let hash = PasswordHash::new(&self.0)
            .change_context_lazy(|| KernelError::Validation {
                entity: "Pass",
                reason: "incorrect format.",
            })?;
        HASHER.verify_password(pass.as_ref().as_bytes(), &hash)
            .change_context_lazy(|| KernelError::Validation {
                entity: "Pass",
                reason: "password does not match.",
            })?;
        Ok(())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[cfg(test)]
mod test {
    use crate::entities::user::pass::Password;

    #[test]
    fn initialize() {
        let pass = Password::new("abc123").unwrap();
        println!("{}", pass)
    }
    
    #[test]
    fn verify() {
        let pass = Password::new("abc123").unwrap();
        pass.verify("abc123").unwrap();
    }
    
    #[test]
    #[should_panic]
    fn verify_failure() {
        let pass = Password::new("abc123").unwrap();
        pass.verify("def456").unwrap();
    }
}