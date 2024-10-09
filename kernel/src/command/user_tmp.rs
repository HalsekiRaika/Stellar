use lutetium::actor::Message;

pub struct TemporalUserSignup;

impl Message for TemporalUserSignup {}

#[derive(Debug, Clone)]
pub enum TemporalUserCommand {
    EnterAddress { address: String },
    Verification2FA { code: String },
    BasicInfoRegistration { name: String, pass: String },
}

impl Message for TemporalUserCommand {}