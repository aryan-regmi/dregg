use iced::{Element, Task};

use crate::views::{screen::Screen, Component};

#[derive(Debug, Default)]
pub struct App {
    /// The current page being displayed.
    pub page: Screen,

    /// The state of the app.
    pub state: State,
}

impl App {
    pub const TITLE: &str = "Dregg";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn view_fixed(&self) -> Element<Message> {
        self.view(())
    }
}

impl Component for App {
    type Message = Message;

    type Context = ();

    type Command = Task<Self::Message>;

    fn view(&self, ctx: Self::Context) -> iced::Element<Self::Message> {
        self.page.view(ctx)
    }

    fn update(&mut self, message: Self::Message) -> Self::Command {
        match message {
            Message::MainMenuButtonPressed => {
                self.page = Screen::Main;
                let _cmd = self.page.update(message);
                Task::none()
            }
            Message::LoadCharacterButtonPressed => {
                self.page = Screen::LoadCharacter;
                let _cmd = self.page.update(message);
                Task::none()
            }
            Message::NewCharacterButtonPressed => {
                self.page = Screen::NewCharacter;
                let _cmd = self.page.update(message);
                Task::none()
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct State {}

/// The messages sent by the `App` component.
#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    LoadCharacterButtonPressed,
    NewCharacterButtonPressed,
}
