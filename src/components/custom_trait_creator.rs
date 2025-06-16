use iced::widget;

#[derive(Debug, Clone)]
pub enum Message {
    /// The trait name was edited.
    NameEdit(String),

    /// Back button pressed.
    BackButtonPressed,
}

pub enum Action {
    /// No action required.
    None,

    /// Cancel adding a new trait.
    Cancel,
}

#[derive(Default, Debug)]
pub struct CustomTraitCreator {
    /// The name of the trait.
    name: String,

    /// The content of the summary text input field.
    summary_editor: widget::text_editor::Content,

    /// The required level to gain access to the trait.
    pub required_level: Option<u8>,

    /// Tags the trait belongs under.
    pub tags: Vec<String>,
}

impl CustomTraitCreator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CustomTraitCreator {
    pub fn view(&self) -> iced::Element<Message> {
        widget::column![widget::button("Back").on_press(Message::BackButtonPressed)].into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameEdit(_) => Action::None,
            Message::BackButtonPressed => Action::Cancel,
        }
    }
}
