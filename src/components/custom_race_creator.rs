use iced::widget;

use crate::{components::race::Race, utils};

#[derive(Debug, Clone)]
pub enum Message {
    /// The race name was edited.
    NameEdit(String),

    /// The race summary was edited.
    SummaryEdit(widget::text_editor::Action),

    /// The race plural name was edited.
    PluralNameEdit(String),

    /// Increase the given ASI value by one.
    IncrementCounter(utils::Attribute),

    /// Decrease the given ASI value by one.
    DecrementCounter(utils::Attribute),

    /// Updates the ASI to the given value.
    UpdateASI((utils::Attribute, String)),

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

    /// Ability score increases provided by the race.
    asi: Vec<utils::ASI>,

    /// Age info for the race.
    age: Option<utils::AgeInfo>,
}

impl CustomRaceCreator {
    pub fn new() -> Self {
        Self {
            name: String::with_capacity(256),
            plural_name: None,
            summary_editor: widget::text_editor::Content::new(),
            asi: utils::ATTRIBUTES
                .iter()
                .map(|attr| utils::ASI {
                    attribute: attr.clone(),
                    value: 0,
                })
                .collect(),
            age: None,
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

            Message::IncrementCounter(attribute) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = asi.value.checked_add(1).unwrap_or_else(|| asi.value);
                }
                Action::None
            }

            Message::DecrementCounter(attribute) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = asi.value.checked_sub(1).unwrap_or_else(|| asi.value);
                }
                Action::None
            }

            Message::UpdateASI((attribute, value)) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = value.parse().unwrap_or_default();
                }
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

        let asi = {
            let mut content = widget::row![widget::container(widget::text("Choose ASIs: "))];

            let mut inner = widget::column![];
            for asi in &self.asi {
                let label = widget::container(widget::text(format!("{}: ", asi.attribute)));
                let input = widget::text_input("0", &asi.value.to_string())
                    .on_input(|value| Message::UpdateASI((asi.attribute, value)));
                let counters = {
                    let increment =
                        widget::button("+").on_press(Message::IncrementCounter(asi.attribute));
                    let decrement =
                        widget::button("-").on_press(Message::DecrementCounter(asi.attribute));
                    widget::row![increment, decrement]
                };
                inner = inner.push(widget::row![label, input, counters]);
                inner = inner.push(widget::vertical_space());
            }
            content = content.push(inner);

            content
        };

        let age = {
            let age = &self.age.clone().unwrap_or_else(|| utils::AgeInfo {
                adult: utils::Age(0),
                lifespan: utils::Age(0),
            });

            widget::row![
                widget::container(widget::text("Age (when considered adult): ")),
                widget::text_input("", &age.adult.to_string()),
                widget::container(widget::text("Age (average lifespan): ")),
            ]
        };

        widget::column![
            title,
            name,
            plural_name,
            summary,
            asi,
            age,
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
