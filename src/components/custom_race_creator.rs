use iced::widget;

use crate::components::race::Race;

#[derive(Debug, Clone)]
pub enum Message {
    /// The race name was edited.
    NameEdit(String),

    /// The race summary was edited.
    SummaryEdit(widget::text_editor::Action),

    /// The race plural name was edited.
    PluralNameEdit(String),

    // TODO: Validate inputs!
    //
    /// The custom race is ready to be created.
    Create,
}

pub enum Action {
    /// No action required.
    None,

    /// Returns to `NewCharacter` page.
    CreateAndReturn(Race),
}

/// The custom race creator component.
#[derive(Default, Debug)]
pub struct CustomRaceCreator {
    /// The name of the race.
    name: String,

    /// The plural name of the race.
    ///
    /// This is used for stringifying certain values.
    plural_name: Option<String>,

    /// The content of the summary text input field.
    summary_editor: widget::text_editor::Content,
    // /// Ability score increases provided by the race.
    // asi: Option<Vec<utils::ASI>>,
}

impl CustomRaceCreator {
    pub fn new() -> Self {
        Self {
            name: String::with_capacity(256),
            plural_name: None,
            summary_editor: widget::text_editor::Content::new(),
            // asi: None,
        }
    }
}

impl CustomRaceCreator {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SummaryEdit(action) => {
                self.summary_editor.perform(action);
                Action::None
            }
            Message::NameEdit(name) => {
                self.name = name;
                Action::None
            }
            Message::PluralNameEdit(plural_name) => {
                self.plural_name = Some(plural_name);
                Action::None
            }
            Message::Create => Action::CreateAndReturn(self.into()),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title =
            widget::container(widget::text("Create Custom Race:").center()).center_x(iced::Fill);

        let name = widget::row![
            widget::container(widget::text("Name: ")),
            widget::text_input("Enter name here...", &self.name).on_input(Message::NameEdit)
        ];

        let plural_name = widget::row![
            widget::container(widget::text("Plural Name: ")),
            widget::text_input(
                "Enter plural name here...",
                self.plural_name.as_ref().unwrap_or_else(|| &self.name)
            )
            .on_input(Message::PluralNameEdit)
        ];

        let summary = widget::row![
            widget::container(widget::text("Summary: ")),
            widget::text_editor(&self.summary_editor).on_action(Message::SummaryEdit),
        ];

        widget::column![
            title,
            name,
            plural_name,
            summary,
            widget::button("Create").on_press(Message::Create)
        ]
        .spacing(5)
        .padding(20)
        .into()
    }
}

impl Into<Race> for &mut CustomRaceCreator {
    fn into(self) -> Race {
        Race {
            name: self.name.clone(),
            plural_name: self
                .plural_name
                .clone()
                .unwrap_or_else(|| self.name.clone()),
        }
    }
}
