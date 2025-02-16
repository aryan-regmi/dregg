use super::main_menu_btn;
use crate::{Message, State};
use iced::{Element, Task};

pub fn view(_: &State) -> Element<Message> {
    main_menu_btn()
}

pub fn update(_: Message) -> Task<Message> {
    Task::none()
}
