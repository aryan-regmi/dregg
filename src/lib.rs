use frontend::{load_char_page, main_page, new_char_page};

pub mod frontend;

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

pub enum Screen {
    Main(main_page::State),
    LoadCharacter(load_char_page::State),
    NewCharacter(new_char_page::State),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Main(main_page::State {})
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Main(main_page::Message),
    LoadCharacter(load_char_page::Message),
    NewCharacter(new_char_page::Message),
}
