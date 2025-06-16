use iced::widget;

use crate::utils;

#[derive(Debug, Clone)]
pub enum Message {
    /// The trait name was edited.
    NameEdit(String),

    /// The trait name was edited.
    SummaryEdit(widget::text_editor::Action),

    /// Back button pressed.
    BackButtonPressed,

    /// Back button pressed.
    CreateButtonPressed,
}

pub enum Action {
    /// No action required.
    None,

    /// Cancel adding a new trait.
    Cancel,

    /// Create and return
    Create(utils::Trait),
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
        let title =
            widget::container(widget::text("Create Custom Race:").center()).center_x(iced::Fill);

        let name = widget::row![
            widget::container(widget::text("Name: ")),
            widget::text_input("Enter name here...", &self.name).on_input(Message::NameEdit)
        ];

        let summary = widget::column![widget::row![
            widget::container(widget::text("Summary: ")),
            widget::text_editor(&self.summary_editor)
                .on_action(Message::SummaryEdit)
                .height(iced::Length::Fixed(200.0))
        ]];

        // TODO: Add Traits fields
        //  - "Add Effect" button:
        //      - Required level input
        //      - Input for tags (separated by comma)
        //      - Dropdown list of trait effects
        //      - Display corresponding inputs, depending on the type

        widget::column![
            title,
            name,
            summary,
            widget::row![
                widget::button("Back").on_press(Message::BackButtonPressed),
                widget::button("Create").on_press(Message::CreateButtonPressed),
            ]
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameEdit(name) => {
                self.name = name;
                Action::None
            }
            Message::SummaryEdit(action) => {
                self.summary_editor.perform(action);
                Action::None
            }
            Message::BackButtonPressed => Action::Cancel,
            Message::CreateButtonPressed => Action::Create(self.into()),
        }
    }
}

impl Into<utils::Trait> for &mut CustomTraitCreator {
    fn into(self) -> utils::Trait {
        utils::Trait {
            name: self.name.clone(),
            summary: self.summary_editor.text(),
            effects: vec![],
            required_level: None,
            tags: vec![],
        }
    }
}
