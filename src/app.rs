use iced::{Element, Task};

use crate::{component::Component, page::Page, pages::new_character_page};

/// The main application.
#[derive(Default)]
pub struct App {
    current_page: Page,
}

impl App {
    /// Returns title of the app.
    pub fn title() -> &'static str {
        "Dregg"
    }
}

impl Component<Message, Task<Message>> for App {
    fn update(&mut self, message: Message) -> Task<Message> {
        let command = self.current_page.update(message);
        match command {
            crate::page::Command::ChangePage(page) => {
                self.current_page = page;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.current_page.view()
    }
}

/// The messages sent by the app.
#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
    LoadCharacterButtonPressed,
}
