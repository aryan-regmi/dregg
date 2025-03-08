#![allow(dead_code)]

use iced::{
    widget::{column, container},
    Element, Task, Theme,
};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed,
}

#[derive(Default, Debug)]
enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

impl Page {
    pub fn view(&self) -> Element<Message> {
        container(column![]).into()
    }
}

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
        match message {
            Message::MainMenuButtonPressed => {
                self.page = Page::Main;
                Task::none()
            }
            Message::LoadCharacterButtonPressed => {
                self.page = Page::LoadCharacter;
                Task::none()
            }
            Message::NewCharacterButtonPressed => {
                self.page = Page::NewCharacter;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.page.view()
    }
}
