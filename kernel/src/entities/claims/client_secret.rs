use std::time::UNIX_EPOCH;
use destructure::Destructure;
use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::KernelError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, Destructure)]
pub struct ClientSecret {
    secret: String,
    expires_at: OffsetDateTime
}

impl ClientSecret {
    pub fn new(secret: impl Into<String>, exp: impl Into<OffsetDateTime>) -> Self {
        Self {
            secret: secret.into(),
            expires_at: exp.into()
        }
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn expires_at(&self) -> &OffsetDateTime {
        &self.expires_at
    }

    pub fn expires_at_as_u64(&self) -> u64 {
        (UNIX_EPOCH - self.expires_at).abs().whole_seconds() as u64
    }

    pub fn verify(&self, _secret: impl Into<String>) -> Result<(), KernelError> {
        todo!()
    }
}

impl From<ClientSecret> for (String, OffsetDateTime) {
    fn from(value: ClientSecret) -> Self {
        (value.secret, value.expires_at)
    }
}

impl Default for ClientSecret {
    fn default() -> Self {
        Self::new(
            Alphanumeric.sample_iter(&mut rand::thread_rng())
                .take(64)
                .map(char::from)
                .collect::<String>(),
            OffsetDateTime::now_utc()
        )
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, Distribution};
    use time::OffsetDateTime;
    use crate::entities::ClientSecret;

    #[test]
    fn test() -> anyhow::Result<()> {
        let code = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(64)
            .map(char::from)
            .collect::<String>();
        let secret = ClientSecret::new(code, OffsetDateTime::now_utc());
        let exp = secret.expires_at_as_u64();
        println!("{:?}", exp);
        Ok(())
    }
}