use iced::widget;

use crate::utils;

#[derive(Debug, Clone)]
pub enum Message {
    /// The trait name was edited.
    NameEdit(String),

    /// The trait name was edited.
    SummaryEdit(widget::text_editor::Action),

    /// Adds an effect to the trait
    AddEffect,

    /// A trait effect was selected in the dropdown.
    EffectSelected(utils::TraitEffect),

    /// Back button pressed.
    BackButtonPressed,

    /// Create button pressed.
    CreateButtonPressed,
}

pub enum Action {
    /// No action required.
    None,

    /// Cancel adding a new trait.
    Cancel,

    /// Create and return the trait.
    Create(utils::Trait),
}

#[derive(Default, Debug)]
pub struct TraitCreator {
    /// The name of the trait.
    name: String,

    /// The content of the summary text input field.
    summary_editor: widget::text_editor::Content,

    /// The required level to gain access to the trait.
    required_level: Option<u8>,

    /// Tags the trait belongs under.
    tags: Vec<String>,

    /// Determines whether or not to display the `Add Effect` page.
    display_add_effect: bool,

    /// The effects of the trait.
    effects: Vec<utils::TraitEffect>,

    /// The trait effect selected in the dropdown.
    effects_dropdown_selection: Option<utils::TraitEffect>,
}

impl TraitCreator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all possible trait effects (for dropdown list).
    fn all_trait_effects() -> Vec<utils::TraitEffect> {
        vec![
            utils::TraitEffect::Vision(utils::Vision::Normal(60)),
            utils::TraitEffect::SavingThrows {
                advantage: utils::Advantage::None,
                kind: utils::SavingThrowsType::Attribute(utils::Attribute::Strength),
            },
            utils::TraitEffect::Resistances(utils::Resistance::Resistance(
                utils::DamageType::Slashing,
            )),
            utils::TraitEffect::Proficiencies(utils::Choice::Single(utils::Proficiency {
                level: utils::ProficiencyLevel::Proficient,
                kind: utils::ProficiencyType::SavingThrows(utils::Attribute::Strength),
                context: None,
            })),
            utils::TraitEffect::Spell(utils::Spell {
                level: 0,
                casting_time: utils::CastingTime::Action(utils::ActionType::Action),
                range: 0,
                components: vec![],
                duration: utils::Duration::Instantaneous,
                effects: vec![],
                upcast: None,
                higher_levels: None,
            }),
            utils::TraitEffect::Action {
                kind: utils::ActionType::Action,
                effects: vec![],
            },
            utils::TraitEffect::HpIncrease(utils::HpIncrease::Max(0)),
            utils::TraitEffect::NoSpeedReduction,
        ]
    }

    /// Handles the trait effect selection.
    fn handle_effect_selection(&self) -> iced::Element<Message> {
        if let Some(selection) = self.effects_dropdown_selection.as_ref() {
            match selection {
                utils::TraitEffect::Vision(vision) => todo!(),
                utils::TraitEffect::SavingThrows { advantage, kind } => todo!(),
                utils::TraitEffect::Resistances(resistance) => todo!(),
                utils::TraitEffect::Proficiencies(choice) => todo!(),
                utils::TraitEffect::Spell(spell) => todo!(),
                utils::TraitEffect::Action { kind, effects } => todo!(),
                utils::TraitEffect::HpIncrease(hp_increase) => todo!(),
                utils::TraitEffect::NoSpeedReduction => todo!(),
            }
        } else {
            widget::column![].into()
        }
    }
}

impl TraitCreator {
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

        let add_effect = widget::button("+ Effect").on_press(Message::AddEffect);

        // TODO: Add Traits fields
        //  - "Add Effect" button:
        //      - Required level input
        //      - Input for tags (separated by comma)
        //      - Dropdown list of trait effects
        //          -
        //              let dropdown = widget::pick_list(races, self.selected_race.as_ref(), |race| {
        //                  Message::RaceSelected(race)
        //              });
        //
        //      - Display corresponding inputs, depending on the type

        let effect_adder = if self.display_add_effect {
            let dropdown = widget::pick_list(
                Self::all_trait_effects(),
                self.effects_dropdown_selection.as_ref(),
                Message::EffectSelected,
            );
            let selection_view = self.handle_effect_selection();
            widget::container(widget::column![dropdown, selection_view])
        } else {
            widget::container(widget::column![])
        };

        widget::column![
            title,
            name,
            summary,
            add_effect,
            effect_adder,
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
            Message::AddEffect => {
                self.display_add_effect = true;
                Action::None
            }
            Message::EffectSelected(trait_effect) => {
                self.effects_dropdown_selection = Some(trait_effect);
                Action::None
            }
            Message::BackButtonPressed => Action::Cancel,
            Message::CreateButtonPressed => Action::Create(self.into()),
        }
    }
}

impl Into<utils::Trait> for &mut TraitCreator {
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
