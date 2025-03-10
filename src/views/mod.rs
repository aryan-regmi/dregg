use iced::Element;

pub mod page_view;
pub mod race_view;

/// A type that can be displayed.
pub trait Component {
    type Message;
    type Context;
    type Command;

    /// Describes how the component is drawn.
    fn view(&self, ctx: Self::Context) -> Element<Self::Message>;

    /// Describes how the component's state is updated.
    fn update(&mut self, message: Self::Message) -> Self::Command;
}
