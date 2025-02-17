use iced::{
    alignment::Horizontal,
    widget::{button, container},
    Element, Length,
};

pub mod load_char_page;
pub mod main_page;
pub mod new_char_page;

/// Returns the button that takes user back to the main menu screen.
pub fn main_menu_btn<'a>() -> Element<'a, main_page::Message> {
    let button = button("Main Menu").on_press(main_page::Message::NewCharacter);
    container(button)
        .padding(20)
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .into()
}
