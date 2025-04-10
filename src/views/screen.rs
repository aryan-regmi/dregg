use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::app;

use super::Component;

/// Represents the various screens of the app.
#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

impl Component for Screen {
    type Message = app::Message;
    type Context = ();
    type Command = ();

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        // Button to return to the main menu.
        let main_menu_btn =
            container(button("Main Menu").on_press(app::Message::MainMenuButtonPressed))
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
                })
                .into();

        match self {
            Screen::Main => self.main_page_view(),
            Screen::LoadCharacter => main_menu_btn,
            Screen::NewCharacter => container(column![main_menu_btn]).padding(0.5).into(),
        }
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}

impl Screen {
    /// Displays the main page.
    fn main_page_view(&self) -> Element<app::Message> {
        container(
            column![
                Self::main_opts_button("New Character", app::Message::NewCharacterButtonPressed),
                Self::main_opts_button("Load Character", app::Message::LoadCharacterButtonPressed),
            ]
            .spacing(20),
        )
        .center(Length::Fill)
        .into()
    }

    // Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: app::Message) -> Element<app::Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }
}
