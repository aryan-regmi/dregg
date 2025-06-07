use iced::{
    widget::{container, row},
    Element, Task,
};

#[derive(Default)]
pub struct App {}

impl App {
    /// Returns title of the app.
    pub fn title() -> &'static str {
        "Dregg"
    }

    /// Updates the state of the app.
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    /// Renders the app.
    pub fn view(&self) -> Element<Message> {
        container(row![]).into()
    }
}

#[derive(Debug)]
pub enum Message {}
