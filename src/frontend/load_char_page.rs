use super::main_menu_btn;
use crate::{Message, Screen, State};
use iced::{Element, Task};

pub fn view(_: &State) -> Element<Message> {
    main_menu_btn()
}

pub fn update(state: &mut State) -> Task<Message> {
    state.screen = Screen::LoadCharacter;
    Task::none()
}
