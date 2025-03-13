use iced::widget::column;

use crate::views::Component;

/// Represents a summary/description.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    /// The main summary.
    pub main: String,

    /// A list of subsections in the form (Title, Content).
    pub subsections: Vec<(String, String)>,
}

impl Component for Summary {
    type Message = ();

    type Context = ();

    type Command = ();

    fn view(&self, _ctx: Self::Context) -> iced::Element<Self::Message> {
        column![].into()
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}
