use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::{app::Message, component::Component};

/// Represents the various pages of the application.
#[derive(Default)]
pub enum Page {
    #[default]
    Main,
    NewCharacter,
    LoadCharacter,
}

impl Page {
    /// Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }
}

impl Component<Message, Command> for Page {
    fn update(&mut self, message: Message) -> Command {
        match message {
            Message::MainMenuButtonPressed => Command::ChangePage(Page::Main),
            Message::NewCharacterButtonPressed => Command::ChangePage(Page::NewCharacter),
            Message::LoadCharacterButtonPressed => Command::ChangePage(Page::LoadCharacter),
        }
    }

    fn view(&self) -> Element<Message> {
        let main_menu_btn = container(button("Main Menu").on_press(Message::MainMenuButtonPressed))
            .padding(20)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 2.0,
                    ..Border::default()
                },
                ..Default::default()
            });

        match self {
            Page::Main => container(
                column![
                    Self::main_opts_button("New Character", Message::NewCharacterButtonPressed),
                    Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed)
                ]
                .spacing(20),
            )
            .center(Length::Fill)
            .into(),
            Page::NewCharacter => container(column![main_menu_btn]).into(),
            Page::LoadCharacter => container(column![main_menu_btn]).into(),
        }
    }
}

/// Represents commands a page can send to the application.
pub enum Command {
    ChangePage(Page),
}
