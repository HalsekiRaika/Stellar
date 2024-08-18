use lutetium::actor::Message;

#[derive(Debug)]
pub enum UserCommand {
    Register { name: String, pass: String },
    Withdrawal { pass: String },
}

impl Message for UserCommand {}