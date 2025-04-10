pub mod pages;
pub mod screen;

use iced::Element;

/// A component to be displayed and updated.
pub trait Component {
    /// Type of the message handled by the component.
    type Message;

    /// Type of parameters for the `view` function.
    type Context;

    /// Type of commands returned by the `update` function.
    type Command;

    /// Describes how the component is drawn.
    fn view(&self, ctx: Self::Context) -> Element<Self::Message>;

    /// Describes how the component's state is updated.
    fn update(&mut self, message: Self::Message) -> Self::Command;
}
