#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub enum Command {
    None,
}

#[derive(Debug)]
pub struct State {}

// TODO: Change output to `Element<Message>`
pub fn view(_: &State) {}

pub fn update(_: Message) -> Command {
    Command::None
}
