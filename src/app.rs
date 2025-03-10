#![allow(dead_code)]

use iced::{Element, Task, Theme};

use crate::{character::Character, views::Component};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed,
}

#[derive(Default, Debug)]
pub enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

#[derive(Default, Debug)]
pub struct App {
    /// The theme of the app.
    theme: Theme,

    /// The current page being displayed.
    page: Page,

    /// The state of the app.
    state: Character,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(&self) -> String {
        String::from("Dregg")
    }

    pub fn view_fixed(&self) -> Element<Message> {
        self.view(())
    }
}

impl Component for App {
    type Message = Message;
    type Context = ();
    type Command = Task<Message>;

    fn view(&self, ctx: Self::Context) -> Element<Self::Message> {
        self.page.view(ctx)
    }

    fn update(&mut self, message: Self::Message) -> Self::Command {
        match message {
            Message::MainMenuButtonPressed => {
                self.page = Page::Main;
                let _cmd = self.page.update(message);
                Task::none()
            }
            Message::LoadCharacterButtonPressed => {
                self.page = Page::LoadCharacter;
                let _cmd = self.page.update(message);
                Task::none()
            }
            Message::NewCharacterButtonPressed => {
                self.page = Page::NewCharacter;
                let _cmd = self.page.update(message);
                Task::none()
            }
        }
    }
}
