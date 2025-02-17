use frontend::new_char_page;

pub mod frontend;

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

#[derive(Default)]
pub enum Screen {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter(new_char_page::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    Main,
    LoadCharacter,
    NewCharacter(new_char_page::Message),
}
