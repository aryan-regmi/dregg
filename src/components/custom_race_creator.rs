use iced::widget;

use crate::{
    components::{
        custom_trait_creator::{self, CustomTraitCreator},
        race::Race,
    },
    utils,
};

#[derive(Debug, Clone)]
pub enum Message {
    /// The race name was edited.
    NameEdit(String),

    /// The race summary was edited.
    SummaryEdit(widget::text_editor::Action),

    /// The race plural name was edited.
    PluralNameEdit(String),

    /// Updates the ASI to the given value.
    UpdateASI((utils::Attribute, String)),

    /// Increase the given ASI value by one.
    IncrementASI(utils::Attribute),

    /// Decrease the given ASI value by one.
    DecrementASI(utils::Attribute),

    /// Updates the selected size of the race.
    SizeSelected(utils::Size),

    /// The adult age of the race was edited.
    AgeAdultEdit(String),

    /// The lifespan of the race was edited.
    AgeLifespanEdit(String),

    /// The height of the race was edited.
    SizeHeightEdit(String),

    /// The height of the race was edited.
    SizeWeightEdit(String),

    /// Updates the speed to the given value.
    UpdateSpeed((utils::Movement, String)),

    /// Increase the given Speed value by one.
    IncrementSpeed(utils::Movement),

    /// Decrease the given Speed value by one.
    DecrementSpeed(utils::Movement),

    /// Display the trait creator.
    DisplayTraitCreator,

    /// Add a trait to the race.
    TraitCreatorView(custom_trait_creator::Message),

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
    age: utils::AgeInfo,

    /// The size for the race.
    size: Option<utils::Size>,

    /// The average height for the race.
    height: Option<String>,

    /// The average weight for the race.
    weight: Option<String>,

    /// The speed for the race.
    speed: Vec<utils::Speed>,

    /// Determines whether a new trait is being added to the race.
    display_trait_creator: bool,

    /// The custom trait creator used to add traits to the race.
    trait_creator: Option<custom_trait_creator::CustomTraitCreator>,
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
            age: utils::AgeInfo {
                adult: None,
                lifespan: None,
            },
            size: Some(utils::Size::Medium),
            height: None,
            weight: None,
            speed: utils::SPEEDS.into(),
            display_trait_creator: false,
            trait_creator: None,
        }
    }

    /// Correctly displays an int value text input.
    fn format_int(value: Option<usize>, default_repr: &str) -> String {
        if let Some(value) = value {
            if value == 0 {
                default_repr.into()
            } else {
                format!("{value}")
            }
        } else {
            default_repr.into()
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
            Message::UpdateASI((attribute, value)) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = value.parse().unwrap_or_default();
                }
                Action::None
            }
            Message::IncrementASI(attribute) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = asi.value.checked_add(1).unwrap_or_else(|| asi.value);
                }
                Action::None
            }
            Message::DecrementASI(attribute) => {
                let asi = self.asi.iter_mut().find(|v| v.attribute == attribute);
                if let Some(asi) = asi {
                    asi.value = asi.value.checked_sub(1).unwrap_or_else(|| asi.value);
                }
                Action::None
            }
            Message::SizeSelected(size) => {
                self.size = Some(size);
                Action::None
            }
            Message::AgeAdultEdit(adult_age) => {
                self.age.adult = Some(utils::Age(adult_age.parse().unwrap_or_else(|_| 0)));
                Action::None
            }
            Message::AgeLifespanEdit(lifespan) => {
                self.age.lifespan = Some(utils::Age(lifespan.parse().unwrap_or_else(|_| 0)));
                Action::None
            }
            Message::SizeHeightEdit(height) => {
                self.height = Some(height);
                Action::None
            }
            Message::SizeWeightEdit(weight) => {
                self.weight = Some(weight);
                Action::None
            }
            Message::UpdateSpeed((movement, value)) => {
                let speed = self.speed.iter_mut().find(|s| s.movement == movement);
                if let Some(speed) = speed {
                    speed.value = value.parse().unwrap_or_default();
                }
                Action::None
            }
            Message::IncrementSpeed(movement) => {
                let speed = self.speed.iter_mut().find(|s| s.movement == movement);
                if let Some(speed) = speed {
                    speed.value = speed.value.checked_add(1).unwrap_or_else(|| 0);
                }
                Action::None
            }
            Message::DecrementSpeed(movement) => {
                let speed = self.speed.iter_mut().find(|s| s.movement == movement);
                if let Some(speed) = speed {
                    speed.value = speed.value.checked_sub(1).unwrap_or_else(|| 0);
                }
                Action::None
            }
            Message::DisplayTraitCreator => {
                self.display_trait_creator = true;
                self.trait_creator = Some(CustomTraitCreator::new());
                Action::None
            }
            Message::TraitCreatorView(message) => {
                if let Some(trait_creator) = &mut self.trait_creator {
                    let command = trait_creator.update(message);
                    match command {
                        custom_trait_creator::Action::None => Action::None,
                        custom_trait_creator::Action::Cancel => {
                            self.display_trait_creator = false;
                            self.trait_creator = None;
                            Action::None
                        }
                    }
                } else {
                    Action::None
                }
            }
            Message::Create => Action::CreateAndReturn(self.into()),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        if self.display_trait_creator {
            widget::column![self
                .trait_creator
                .as_ref()
                .unwrap()
                .view()
                .map(Message::TraitCreatorView)]
            .into()
        } else {
            let title = widget::container(widget::text("Create Custom Race:").center())
                .center_x(iced::Fill);

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

            let summary = widget::column![widget::row![
                widget::container(widget::text("Summary: ")),
                widget::text_editor(&self.summary_editor)
                    .on_action(Message::SummaryEdit)
                    .height(iced::Length::Fixed(200.0))
            ]];

            let asi = {
                let mut content = widget::column![widget::container(widget::text("Choose ASIs: "))];

                for asi in &self.asi {
                    let label = widget::container(widget::text(format!("{}: ", asi.attribute)));
                    let input_str = Self::format_int(Some(asi.value as usize), "");
                    let input = widget::text_input("0", &input_str)
                        .on_input(|value| Message::UpdateASI((asi.attribute, value)));
                    let counters = {
                        let increment =
                            widget::button("+").on_press(Message::IncrementASI(asi.attribute));
                        let decrement =
                            widget::button("-").on_press(Message::DecrementASI(asi.attribute));
                        widget::row![increment, decrement]
                    };
                    content = content.push(widget::row![label, input, counters]);
                }

                content
            };

            let age = {
                let adult_age = Self::format_int(self.age.adult.clone().map(|v| v.0), "");
                let lifespan = Self::format_int(self.age.lifespan.clone().map(|v| v.0), "");

                widget::row![
                    widget::container(widget::text("Age (when considered adult): ")),
                    widget::text_input("", &adult_age).on_input(Message::AgeAdultEdit),
                    widget::container(widget::text("Age (average lifespan): ")),
                    widget::text_input("", &lifespan.to_string())
                        .on_input(Message::AgeLifespanEdit),
                ]
            };

            let size = {
                let mut content = widget::row![widget::container(widget::text("Size: ")),];

                // Size category
                {
                    let mut inner_col = widget::column![];
                    for size in utils::SIZES {
                        let radio =
                            widget::radio(size.to_string(), size, self.size, Message::SizeSelected);
                        inner_col = inner_col.push(radio);
                    }
                    content = content.push(inner_col);
                }

                // Height
                {
                    let label = widget::container(widget::text("Height (ft): "));
                    let input = widget::text_input(
                        "",
                        &self.height.clone().unwrap_or_else(|| String::new()),
                    )
                    .on_input(Message::SizeHeightEdit);
                    content = content.push(widget::row![label, input]);
                }

                // Weight
                {
                    let label = widget::container(widget::text("Weight (lbs): "));
                    let input = widget::text_input(
                        "",
                        &self.weight.clone().unwrap_or_else(|| String::new()),
                    )
                    .on_input(Message::SizeWeightEdit);
                    content = content.push(widget::row![label, input]);
                }

                content
            };

            let speed = {
                let mut content =
                    widget::column![widget::container(widget::text("Choose Speeds: "))];

                for speed in &self.speed {
                    let label = widget::container(widget::text(format!("{}: ", speed.to_string())));
                    let input_str = Self::format_int(Some(speed.value as usize), "");
                    let input = widget::text_input("0", &input_str)
                        .on_input(|value| Message::UpdateSpeed((speed.movement, value)));
                    let counters = {
                        let increment =
                            widget::button("+").on_press(Message::IncrementSpeed(speed.movement));
                        let decrement =
                            widget::button("-").on_press(Message::DecrementSpeed(speed.movement));
                        widget::row![increment, decrement]
                    };
                    content = content.push(widget::row![label, input, counters]);
                }

                content
            };

            // TODO: Add Traits fields
            //  - "Add Effect" button:
            //      - Name and summary inputs
            //      - Required level input
            //      - Input for tags (separated by comma)
            //      - Dropdown list of trait effects
            //      - Display corresponding inputs, depending on the type

            let traits =
                widget::row![widget::button("Add Effect").on_press(Message::DisplayTraitCreator)];

            widget::scrollable(
                widget::column![
                    title,
                    name,
                    plural_name,
                    summary,
                    asi,
                    age,
                    size,
                    speed,
                    traits,
                    widget::button("Create").on_press(Message::Create)
                ]
                .spacing(5)
                .padding(20),
            )
            .into()
        }
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
