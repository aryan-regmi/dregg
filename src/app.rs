use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Element, Length, Task, Theme,
};

use crate::views::new_character_page::{self, NewCharacterPage};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
}

#[derive(Default, Debug)]
enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter(NewCharacterPage),
}

impl Page {
    fn update(&self, message: Message) -> (Self, Task<Message>) {
        match message {
            Message::MainMenuButtonPressed => (Page::Main, Task::none()),
            Message::LoadCharacterButtonPressed => (Page::LoadCharacter, Task::none()),
            Message::NewCharacterButtonPressed(_msg) => {
                (Page::NewCharacter(NewCharacterPage::new()), Task::none())
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let main_menu_btn = container(button("Main Menu").on_press(Message::MainMenuButtonPressed))
            .padding(20)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .into();

        match self {
            Page::Main => container(
                column![
                    Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed),
                    Self::main_opts_button(
                        "New Character",
                        Message::NewCharacterButtonPressed(new_character_page::Message::default())
                    ),
                ]
                .spacing(20),
            )
            .center(Length::Fill)
            .into(),
            Page::LoadCharacter => main_menu_btn,
            Page::NewCharacter(page) => container(column![
                page.view().map(Message::NewCharacterButtonPressed),
                main_menu_btn
            ])
            .into(),
        }
    }

    /// Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct App {
    /// The theme of the app.
    theme: Theme,

    /// The current page being displayed.
    page: Page,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            page: Page::default(),
        }
    }

    pub fn title(&self) -> String {
        String::from("Dregg")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let (page, task) = self.page.update(message);
        self.page = page;
        task
    }

    pub fn view(&self) -> Element<Message> {
        self.page.view()
    }
}
