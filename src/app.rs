#![allow(dead_code)]

use iced::{Element, Task, Theme};

use crate::{
    character::Character,
    views::{new_character_component, Component, NewCharacterComponent, RaceComponent},
};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed(new_character_component::Message),
}

#[derive(Default, Debug)]
pub enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter(NewCharacterComponent),
}

#[derive(Default, Debug)]
pub struct App {
    /// The theme of the app.
    theme: Theme, // TODO: Implement a theme picker in the `Main` page!

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
            Message::NewCharacterButtonPressed(msg) => {
                let mut page = NewCharacterComponent::new(&self.state.race);
                let command = page.update(msg);
                match command {
                    new_character_component::Command::None => {
                        self.page = Page::NewCharacter(page);
                        Task::none()
                    }
                    new_character_component::Command::RaceSelected(race) => {
                        let race_component: RaceComponent = race.into();
                        self.state.race = race_component.into();
                        self.page =
                            Page::NewCharacter(NewCharacterComponent::new(&self.state.race));
                        Task::none()
                    }
                }
            }
        }
    }
}
