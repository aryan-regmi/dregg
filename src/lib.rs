use frontend::new_char_page::NewCharacterPage;

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
    NewCharacter(NewCharacterPage),
}

#[derive(Debug, Clone)]
pub enum Message {
    Main,
    LoadCharacter,
    NewCharacter,
}
