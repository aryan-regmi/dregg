#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub enum Action {
    None,
}

pub struct State {}

// TODO: Change output to `Element<Message>`
pub fn view(_: &State) {}

pub fn update(_: Message) -> Action {
    Action::None
}
