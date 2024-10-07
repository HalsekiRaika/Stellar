use lutetium::actor::Message;

#[derive(Debug, Clone)]
pub enum TemporalUserCommand {
    AddressRegistration { address: String },
    Verification2FA { code: String },
    BasicInfoRegistration { name: String, pass: String },
}

impl Message for TemporalUserCommand {}