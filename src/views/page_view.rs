use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::{
    app::{Message, Page},
    views::Component,
};

impl Page {
    /// Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }

    /// Displays the main page.
    fn main_page(&self) -> Element<Message> {
        container(
            column![
                Self::main_opts_button("New Character", Message::NewCharacterButtonPressed),
                Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed),
            ]
            .spacing(20),
        )
        .center(Length::Fill)
        .into()
    }
}

impl Component for Page {
    type Message = Message;
    type Context = ();
    type Command = ();

    fn view(&self, _ctx: Self::Context) -> Element<Self::Message> {
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
            })
            .into();

        match self {
            Page::Main => self.main_page(),
            Page::LoadCharacter => main_menu_btn,
            Page::NewCharacter => main_menu_btn,
        }
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}
