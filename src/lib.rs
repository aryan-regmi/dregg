use frontend::{load_char_page, new_char_page};

pub mod frontend;

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

#[derive(Default)]
pub enum Screen {
    #[default]
    Main,
    LoadCharacter(load_char_page::State),
    NewCharacter(new_char_page::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    Main,
    LoadCharacter(load_char_page::Message),
    NewCharacter(new_char_page::Message),
}
