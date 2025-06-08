use iced::Element;

/// Represents a component that has `update` and `view` methods.
pub trait Component<'a, Message: 'a, Command> {
    /// Update the component's state.
    fn update(&'a mut self, message: Message) -> Command;

    /// Render the component.
    fn view(&'a self) -> Element<Message>;
}
