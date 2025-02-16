use super::main_menu_btn;
use crate::{Message, Screen, State};
use iced::{
    widget::{column, container, pane_grid, text, PaneGrid},
    Element, Task,
};

pub enum NewCharPagePane {
    Menu,
    Main,
}

pub fn view(state: &State) -> Element<Message> {
    let panes = pane_grid(&state.panes, |pane, state, is_maximized| {
        pane_grid::Content::new(match state {
            NewCharPagePane::Menu => text("Menu Items"),
            NewCharPagePane::Main => text("Main window"),
        })
    });

    container(column![panes, main_menu_btn()]).into()
}

pub fn update(state: &mut State) -> Task<Message> {
    state.screen = Screen::NewCharacter;
    Task::none()
}
