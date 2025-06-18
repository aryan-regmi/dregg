use iced::widget;

use crate::utils;

#[derive(Debug, Clone)]
pub enum Message {
    /// The vision type has been selected.
    VisionRadioSelected(utils::Vision),

    /// The vision value has been updated.
    VisionInputEdit(String),

    /// The saving throw advantage type has been selected.
    SavingThrowAdvantageSelected(utils::Advantage),

    /// The saving throw type has been selected.
    SavingThrowTypeSelected(utils::SavingThrowType),

    /// The saving throw attribute has been selected.
    SavingThrowAttributeSelected(utils::Attribute),

    /// The saving throw damage has been selected.
    SavingThrowDamageSelected(utils::DamageType),

    /// The resistance type has been selected.
    ResistanceTypeSelected(utils::Resistance),

    /// The resistance damage type has been selected.
    ResistanceDamageSelected(utils::DamageType),

    /// The proficiency level has been selected.
    ProficiencyLevelSelected(utils::ProficiencyLevel),

    /// Back button pressed.
    BackButtonPressed,

    /// Create button pressed.
    CreateButtonPressed,
}

pub enum Action {
    /// No action required.
    None,

    /// Cancel adding a new trait effect.
    Cancel,

    /// Create and return the trait effect.
    Create(utils::TraitEffect),
}

#[derive(Debug)]
pub struct TraitEffectCreator {
    /// The trait effect to display.
    effect: utils::TraitEffect,

    /// Currently selected vision radio button.
    selected_vision: Option<utils::Vision>,

    /// Currently selected saving throw advantage radio button.
    selected_saving_throw_advantage: Option<utils::Advantage>,

    /// Currently selected saving throw type radio button.
    selected_saving_throw_type: Option<utils::SavingThrowType>,

    /// Currently selected saving throw attribute radio button.
    selected_saving_throw_attribute: Option<utils::Attribute>,

    /// Currently selected saving throw damage radio button.
    selected_saving_throw_damage: Option<utils::DamageType>,

    /// Currently selected resistance radio button.
    selected_resistance_type: Option<utils::Resistance>,

    /// Currently selected resistance damage type radio button.
    selected_resistance_damage: Option<utils::DamageType>,

    /// Currently selected proficiency level radio button.
    selected_proficiency_level: Option<utils::ProficiencyLevel>,
}

impl TraitEffectCreator {
    /// Creates a new `TraitEffectCreator`.
    pub fn new(effect: utils::TraitEffect) -> Self {
        Self {
            effect,
            selected_vision: None,
            selected_saving_throw_advantage: None,
            selected_saving_throw_type: None,
            selected_saving_throw_attribute: None,
            selected_saving_throw_damage: None,
            selected_resistance_type: None,
            selected_resistance_damage: None,
            selected_proficiency_level: None,
        }
    }

    /// Displays the vision options.
    fn display_vision(&self) -> iced::Element<Message> {
        let mut inner = widget::row![];
        for vision in Self::all_visions() {
            let radio = widget::radio(
                vision.to_string(),
                vision,
                self.selected_vision,
                Message::VisionRadioSelected,
            );
            inner = inner.push(widget::container(radio));
        }

        if let Some(selected_vision) = &self.selected_vision {
            let input_str = utils::format_int(Some(selected_vision.value() as usize), "");
            let input = widget::text_input("0", &input_str).on_input(Message::VisionInputEdit);
            inner = inner.push(widget::container(input));
        }

        widget::container(inner).into()
    }

    /// Displays the saving throw options.
    fn display_saving_throws(&self) -> iced::Element<Message> {
        let mut inner = widget::row![];

        // Advantage radio buttons
        let mut adv_radios = widget::column![];
        for adv in Self::all_advantage_types() {
            let radio = widget::radio(
                adv.to_string(),
                adv,
                self.selected_saving_throw_advantage,
                Message::SavingThrowAdvantageSelected,
            );
            adv_radios = adv_radios.push(radio);
        }
        inner = inner.push(adv_radios);

        // Saving throw type radio buttons
        let mut kind_radios = widget::column![];
        for kind in Self::all_saving_throw_types() {
            let radio = widget::radio(
                kind.to_string(),
                kind,
                self.selected_saving_throw_type,
                Message::SavingThrowTypeSelected,
            );
            kind_radios = kind_radios.push(radio);
        }
        inner = inner.push(kind_radios);

        if let Some(saving_throw_type) = &self.selected_saving_throw_type {
            match saving_throw_type {
                // Attribute radio buttons
                utils::SavingThrowType::Attribute(_) => {
                    let mut attr_radios = widget::column![];
                    for attr in utils::ATTRIBUTES {
                        if attr != utils::Attribute::Any {
                            let radio = widget::radio(
                                attr.to_string(),
                                attr,
                                self.selected_saving_throw_attribute,
                                Message::SavingThrowAttributeSelected,
                            );
                            attr_radios = attr_radios.push(radio);
                        }
                    }
                    inner = inner.push(attr_radios);
                }

                // Damage radio buttons
                utils::SavingThrowType::Damage(_) => {
                    let mut dmg_radios = widget::column![];
                    for dmg in Self::all_damage_types() {
                        let radio = widget::radio(
                            &format!("{:?}", dmg),
                            dmg,
                            self.selected_saving_throw_damage,
                            Message::SavingThrowDamageSelected,
                        );
                        dmg_radios = dmg_radios.push(radio);
                    }
                    inner = inner.push(dmg_radios);
                }
            }
        }

        widget::container(inner).into()
    }

    /// Displays the resistance options.
    fn display_resistances(&self) -> iced::Element<Message> {
        let mut inner = widget::row![];

        // Resistance radio buttons
        for resistance in Self::all_resistance_types() {
            let radio = widget::radio(
                resistance.to_string(),
                resistance,
                self.selected_resistance_type,
                Message::ResistanceTypeSelected,
            );
            inner = inner.push(radio);
        }

        // Damage radio buttons
        let mut dmg_radios = widget::column![];
        for dmg in Self::all_damage_types() {
            let radio = widget::radio(
                &format!("{:?}", dmg),
                dmg,
                self.selected_resistance_damage,
                Message::ResistanceDamageSelected,
            );
            dmg_radios = dmg_radios.push(radio);
        }
        inner = inner.push(dmg_radios);

        widget::container(inner).into()
    }

    /// Displays the proficiency options.
    fn display_proficiencies(&self) -> iced::Element<Message> {
        let mut inner = widget::row![];

        let mut level_radios = widget::column![];
        for proficiency in Self::all_proficiency_levels() {
            let radio = widget::radio(
                &format!("{:?}", proficiency),
                proficiency,
                self.selected_proficiency_level,
                Message::ProficiencyLevelSelected,
            );
            level_radios = level_radios.push(radio);
        }
        inner = inner.push(level_radios);

        widget::container(inner).into()
    }

    /// Returns a list of all vision types.
    fn all_visions() -> Vec<utils::Vision> {
        vec![
            utils::Vision::Normal(0),
            utils::Vision::Darkvision(0),
            utils::Vision::Truesight(0),
            utils::Vision::DevilsSight(0),
        ]
    }

    /// Returns a list of all advantage types.
    fn all_advantage_types() -> Vec<utils::Advantage> {
        vec![utils::Advantage::Advantage, utils::Advantage::Disadvantage]
    }

    /// Returns a list of all damage types.
    fn all_damage_types() -> Vec<utils::DamageType> {
        vec![
            utils::DamageType::Acid,
            utils::DamageType::Bludgeoning,
            utils::DamageType::Cold,
            utils::DamageType::Fire,
            utils::DamageType::Force,
            utils::DamageType::Lightning,
            utils::DamageType::Necrotic,
            utils::DamageType::Piercing,
            utils::DamageType::Poison,
            utils::DamageType::Psychic,
            utils::DamageType::Radiant,
            utils::DamageType::Slashing,
            utils::DamageType::Thunder,
        ]
    }

    /// Returns a list of all damage types.
    fn all_saving_throw_types() -> Vec<utils::SavingThrowType> {
        vec![
            utils::SavingThrowType::Attribute(utils::Attribute::Any),
            utils::SavingThrowType::Damage(utils::DamageType::Acid),
        ]
    }

    /// Returns a list of all resistance types.
    fn all_resistance_types() -> Vec<utils::Resistance> {
        vec![
            utils::Resistance::Resistance(utils::DamageType::Acid),
            utils::Resistance::Vulnerability(utils::DamageType::Acid),
        ]
    }

    /// Returns a list of all proficiency types.
    fn all_proficiency_levels() -> Vec<utils::ProficiencyLevel> {
        vec![
            utils::ProficiencyLevel::Proficient,
            utils::ProficiencyLevel::Expertise,
        ]
    }
}

impl TraitEffectCreator {
    pub fn view(&self) -> iced::Element<Message> {
        let mut content = widget::column![];

        match &self.effect {
            utils::TraitEffect::Vision(_) => content = content.push(self.display_vision()),
            utils::TraitEffect::SavingThrows { .. } => {
                content = content.push(self.display_saving_throws())
            }
            utils::TraitEffect::Resistances(_) => {
                content = content.push(self.display_resistances())
            }
            utils::TraitEffect::Proficiencies(_) => {
                content = content.push(self.display_proficiencies())
            }
            utils::TraitEffect::Spell(_) => todo!(),
            utils::TraitEffect::Action { .. } => todo!(),
            utils::TraitEffect::HpIncrease(_) => todo!(),
            utils::TraitEffect::NoSpeedReduction => todo!(),
        }

        // Add navigation buttons
        {
            let navigation_buttons = widget::container(widget::row![
                widget::button("Back").on_press(Message::BackButtonPressed),
                widget::button("Create").on_press(Message::CreateButtonPressed),
            ]);
            content = content.push(navigation_buttons);
        }

        widget::container(content).into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::VisionRadioSelected(vision) => {
                self.selected_vision = Some(vision);
                self.effect = utils::TraitEffect::Vision(vision);
                Action::None
            }
            Message::VisionInputEdit(value) => {
                if let Some(vision) = &mut self.selected_vision {
                    vision.set_value(value.parse().unwrap_or_default());
                }
                Action::None
            }
            Message::SavingThrowAdvantageSelected(advantage) => {
                self.selected_saving_throw_advantage = Some(advantage);
                Action::None
            }
            Message::SavingThrowTypeSelected(saving_throw_type) => {
                self.selected_saving_throw_type = Some(saving_throw_type);
                Action::None
            }
            Message::SavingThrowAttributeSelected(attribute) => {
                self.selected_saving_throw_attribute = Some(attribute);
                Action::None
            }
            Message::SavingThrowDamageSelected(damage_type) => {
                self.selected_saving_throw_damage = Some(damage_type);
                Action::None
            }
            Message::ResistanceTypeSelected(resistance) => {
                self.selected_resistance_type = Some(resistance);
                Action::None
            }
            Message::ResistanceDamageSelected(damage_type) => {
                self.selected_resistance_damage = Some(damage_type);
                Action::None
            }
            Message::ProficiencyLevelSelected(proficiency_level) => {
                self.selected_proficiency_level = Some(proficiency_level);
                Action::None
            }
            Message::BackButtonPressed => Action::Cancel,
            Message::CreateButtonPressed => Action::Create(self.effect.clone()),
        }
    }
}
