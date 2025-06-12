use iced::widget;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(widget::text_editor::Action),

    Create,
}

pub enum Action {
    /// No action required.
    None,

    /// Update the summary text.
    UpdateSummaryText(String),

    /// Returns to `NewCharacter` page.
    CreateAndReturn,
}

/// The custom race creator component.
#[derive(Default, Debug)]
pub struct CustomRaceCreator {
    /// The name of the race.
    pub name: Option<String>,

    /// The plural name of the race.
    ///
    /// This is used for stringifying certain values.
    pub plural_name: Option<String>,

    pub summary_editor: widget::text_editor::Content,
}

impl CustomRaceCreator {
    pub fn new(summary_text: &str) -> Self {
        let summary_editor = widget::text_editor::Content::with_text(summary_text);
        Self {
            name: None,
            plural_name: None,
            summary_editor,
        }
    }
}

impl CustomRaceCreator {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Edit(action) => {
                self.summary_editor.perform(action);
                Action::UpdateSummaryText(self.summary_editor.text())
            }
            Message::Create => Action::CreateAndReturn,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        widget::column![
            widget::text_editor(&self.summary_editor).on_action(Message::Edit),
            widget::button("Create").on_press(Message::Create)
        ]
        .padding(20)
        .into()
    }
}
