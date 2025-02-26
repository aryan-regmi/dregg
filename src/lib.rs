#![allow(unused)]

use frontend::{load_char_page, new_char_page, race::RaceName};

pub mod frontend;

#[derive(Default, Debug)]
pub struct State {
    pub screen: Screen,
    pub selected_race: Option<RaceName>,
}

#[derive(Default, Debug)]
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
