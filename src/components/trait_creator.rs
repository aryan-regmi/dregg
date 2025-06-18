use iced::widget;

use crate::{
    components::trait_effect_creator::{self, TraitEffectCreator},
    utils,
};

#[derive(Debug, Clone)]
pub enum Message {
    /// The trait name was edited.
    NameEdit(String),

    /// The trait name was edited.
    SummaryEdit(widget::text_editor::Action),

    /// Display the trait effect creator.
    DisplayTraitEffectCreator,

    /// Adds an effect to the trait.
    TraitEffectCreatorView(trait_effect_creator::Message),

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
    _required_level: Option<u8>,

    /// Tags the trait belongs under.
    _tags: Vec<String>,

    /// Determines whether a new effect is being added to the trait.
    display_effect_creator: bool,

    /// The custom trait effect creator used to add effects to the trait.
    effect_creator: Option<trait_effect_creator::TraitEffectCreator>,

    /// The trait effect selected in the dropdown.
    effects_dropdown_selection: Option<utils::TraitEffect>,

    /// The effects of the trait.
    effects: Vec<utils::TraitEffect>,
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
                advantage: utils::Advantage::Advantage,
                kind: utils::SavingThrowType::Attribute(utils::Attribute::Strength),
            },
            utils::TraitEffect::Resistances(utils::Resistance::Resistance(
                utils::DamageType::Slashing,
            )),
            utils::TraitEffect::Proficiencies(utils::Choice::AllOf(vec![utils::Proficiency {
                level: utils::ProficiencyLevel::Proficient,
                kind: utils::ProficiencyType::SavingThrows(utils::Attribute::Strength),
                context: None,
            }])),
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
}

impl TraitCreator {
    pub fn view(&self) -> iced::Element<Message> {
        if self.display_effect_creator {
            let dropdown = widget::pick_list(
                Self::all_trait_effects(),
                self.effects_dropdown_selection.as_ref(),
                Message::EffectSelected,
            );

            let effect_creator = if let Some(effect_creator) = &self.effect_creator {
                effect_creator.view().map(Message::TraitEffectCreatorView)
            } else {
                widget::column![].into()
            };

            widget::column![dropdown, effect_creator].into()
        } else {
            let title = widget::container(widget::text("Create Custom Race:").center())
                .center_x(iced::Fill);

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

            let add_effects = widget::container(widget::column![
                widget::button("Add Effect").on_press(Message::DisplayTraitEffectCreator)
            ]);

            let effects = if self.effects.is_empty() {
                widget::container(widget::column![])
            } else {
                let mut content = widget::column![widget::text("Effects: ")];
                for effect in &self.effects {
                    let info = widget::text(format!("{:?}", effect));
                    content = content.push(info);
                }

                widget::container(content)
            };

            widget::column![
                title,
                name,
                summary,
                add_effects,
                effects,
                widget::row![
                    widget::button("Back").on_press(Message::BackButtonPressed),
                    widget::button("Create").on_press(Message::CreateButtonPressed),
                ]
            ]
            .into()
        }
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
            Message::EffectSelected(trait_effect) => {
                self.effect_creator = Some(TraitEffectCreator::new(trait_effect.clone()));
                self.effects_dropdown_selection = Some(trait_effect);
                Action::None
            }
            Message::DisplayTraitEffectCreator => {
                self.display_effect_creator = true;
                Action::None
            }
            Message::TraitEffectCreatorView(message) => {
                if let Some(effect_creator) = &mut self.effect_creator {
                    match effect_creator.update(message) {
                        trait_effect_creator::Action::None => {}
                        trait_effect_creator::Action::Cancel => {
                            self.display_effect_creator = false;
                        }
                        trait_effect_creator::Action::Create(trait_effect) => {
                            self.effects.push(trait_effect);
                            self.display_effect_creator = false;
                        }
                    }
                }
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
