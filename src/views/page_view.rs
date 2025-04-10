use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::{
    app::{Message, Page},
    views::{new_character_component, Component},
};

impl<'a> Component for Page<'a> {
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
            Page::NewCharacter(page) => container(column![
                page.view(()).map(Message::NewCharacterButtonPressed),
                main_menu_btn,
            ])
            .padding(0.5)
            .into(),
        }
    }

    /// WARNING: THE UPDATE FUNCTION FOR A PAGE DOESN'T DO ANYTHING.
    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}

impl<'a> Page<'a> {
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
                Self::main_opts_button(
                    "New Character",
                    Message::NewCharacterButtonPressed(new_character_component::Message::default())
                ),
                Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed),
            ]
            .spacing(20),
        )
        .center(Length::Fill)
        .into()
    }
}
