use crate::entities::FactorCode;

#[derive(Debug, Clone)]
pub enum TemporalUserEvent {
    AddressRegistered { mfa: FactorCode },
    Verified2FA,
    Registration,
}
