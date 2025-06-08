use iced::Element;

pub trait Component<Message, Command> {
    /// Update the component's state.
    fn update(&mut self, message: Message) -> Command;

    /// Render the component.
    fn view(&self) -> Element<Message>;
}
