use crate::entities::{FactorCode, UserId};

#[derive(Debug, Clone)]
pub enum TemporalUserEvent {
    AcceptedAddress { code: FactorCode },
    Verified2FA,
    Registration { user_id: UserId },
}
