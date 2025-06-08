use iced::{Element, Task};

use crate::{component::Component, page::Page, pages::new_character_page, race::Race};

/// The main application.
#[derive(Default)]
pub struct App {
    current_page: Page,

    new_character_page_props: NewCharacterPageProps,
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
                self.current_page = Page::new(page, self.new_character_page_props.clone());
                Task::none()
            }
            crate::page::Command::UpdateSelectedRace(race) => {
                self.new_character_page_props.selected_race = race;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.current_page.view()
    }
}

#[derive(Default, Clone)]
pub struct NewCharacterPageProps {
    pub selected_race: Option<Race>,
}

/// The messages sent by the app.
#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
    LoadCharacterButtonPressed,
}
