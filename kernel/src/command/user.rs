use lutetium::actor::Message;

#[derive(Debug)]
pub enum UserCommand {
    Register { name: String, pass: String },
    Withdrawal,
}

impl Message for UserCommand {}