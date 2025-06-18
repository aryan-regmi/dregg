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

    /// The proficiency type has been selected.
    ProficiencyTypeSelected(utils::ProficiencyType),

    /// The proficiency armor type has been selected.
    ProficiencyArmorTypeSelected(utils::ArmorType),

    /// The proficiency armor type has been selected.
    ProficiencyWeaponTypeSelected(utils::WeaponType),

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

/// Represents the vision radio buttons.
#[derive(Debug, Default)]
struct VisionRadios(Option<utils::Vision>);

/// Represents the saving throw radio buttons.
#[derive(Debug, Default)]
struct SavingThrowRadios {
    advantage: Option<utils::Advantage>,
    kind: Option<utils::SavingThrowType>,
    attribute: Option<utils::Attribute>,
    damage_type: Option<utils::DamageType>,
}

/// Represents the resistance radio buttons.
#[derive(Debug, Default)]
struct ResistanceRadios {
    /// Currently selected resistance.
    kind: Option<utils::Resistance>,

    /// Currently selected resistance damage type.
    damage_type: Option<utils::DamageType>,
}

/// Represents the proficiency radio buttons.
#[derive(Debug, Default)]
struct ProficiencyRadios {
    /// Currently selected proficiency level.
    level: Option<utils::ProficiencyLevel>,

    /// Currently selected proficiency type.
    kind: Option<utils::ProficiencyType>,

    /// Currently selected armor type.
    armor_type: Option<utils::ArmorType>,

    /// Currently selected weapon type.
    weapon_type: Option<utils::WeaponType>,
}

// TODO: Extract `selected` props into structs
#[derive(Debug)]
pub struct TraitEffectCreator {
    /// The trait effect to display.
    effect: utils::TraitEffect,

    /// Vision radio button.
    vision_radios: VisionRadios,

    /// Saving throw radio buttons.
    saving_throw_radios: SavingThrowRadios,

    /// Resistance radio buttons.
    resistance_radios: ResistanceRadios,

    /// Proficiency radio buttons.
    proficiency_radios: ProficiencyRadios,
}

impl TraitEffectCreator {
    /// Creates a new `TraitEffectCreator`.
    pub fn new(effect: utils::TraitEffect) -> Self {
        Self {
            effect,
            vision_radios: Default::default(),
            saving_throw_radios: Default::default(),
            resistance_radios: Default::default(),
            proficiency_radios: Default::default(),
        }
    }

    /// Displays the vision options.
    fn display_vision(&self) -> iced::Element<Message> {
        let mut inner = widget::row![];
        for vision in Self::all_visions() {
            let radio = widget::radio(
                vision.to_string(),
                vision,
                self.vision_radios.0,
                Message::VisionRadioSelected,
            );
            inner = inner.push(widget::container(radio));
        }

        if let Some(selected_vision) = &self.vision_radios.0 {
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
                self.saving_throw_radios.advantage,
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
                self.saving_throw_radios.kind,
                Message::SavingThrowTypeSelected,
            );
            kind_radios = kind_radios.push(radio);
        }
        inner = inner.push(kind_radios);

        if let Some(saving_throw_type) = &self.saving_throw_radios.kind {
            match saving_throw_type {
                // Attribute radio buttons
                utils::SavingThrowType::Attribute(_) => {
                    let mut attr_radios = widget::column![];
                    for attr in utils::ATTRIBUTES {
                        if attr != utils::Attribute::Any {
                            let radio = widget::radio(
                                attr.to_string(),
                                attr,
                                self.saving_throw_radios.attribute,
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
                            self.saving_throw_radios.damage_type,
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
        let mut resist_radios = widget::column![];
        for resistance in Self::all_resistance_types() {
            let radio = widget::radio(
                resistance.to_string(),
                resistance,
                self.resistance_radios.kind,
                Message::ResistanceTypeSelected,
            );
            resist_radios = resist_radios.push(radio);
        }
        inner = inner.push(resist_radios);

        // Damage radio buttons
        let mut dmg_radios = widget::column![];
        for dmg in Self::all_damage_types() {
            let radio = widget::radio(
                &format!("{:?}", dmg),
                dmg,
                self.resistance_radios.damage_type,
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

        // Level radio buttons
        let mut level_radios = widget::column![];
        for proficiency in Self::all_proficiency_levels() {
            let radio = widget::radio(
                &format!("{:?}", proficiency),
                proficiency,
                self.proficiency_radios.level,
                Message::ProficiencyLevelSelected,
            );
            level_radios = level_radios.push(radio);
        }
        inner = inner.push(level_radios);

        // Type radio buttons
        let mut type_radios = widget::column![];
        for kind in Self::all_proficiency_types() {
            let radio = widget::radio(
                kind.to_string(),
                kind,
                self.proficiency_radios.kind,
                Message::ProficiencyTypeSelected,
            );
            type_radios = type_radios.push(radio);
        }
        inner = inner.push(type_radios);

        // Display proficiency sub-radios, matching on the type
        if let Some(kind) = &self.proficiency_radios.kind {
            match kind {
                utils::ProficiencyType::Armor(_) => {
                    let mut armor_radios = widget::column![];
                    for armor in Self::all_armor_types() {
                        let radio = widget::radio(
                            &format!("{:?}", armor),
                            armor,
                            self.proficiency_radios.armor_type,
                            Message::ProficiencyArmorTypeSelected,
                        );
                        armor_radios = armor_radios.push(radio);
                    }
                    inner = inner.push(armor_radios);
                }

                utils::ProficiencyType::Weapons(_) => {
                    let mut weapon_radios = widget::column![];
                    let mut weapon_radios2 = widget::column![];
                    let weapons = Self::all_weapon_types();
                    let num_weapons = weapons.len();
                    for (i, weapon) in weapons.into_iter().enumerate() {
                        let label = match weapon {
                            utils::WeaponType::SimpleMelee => "Simple Melee",
                            utils::WeaponType::SimpleRanged => "Simple Ranged",
                            utils::WeaponType::MartialMelee => "Martial Melee",
                            utils::WeaponType::MaritalRanged => "Marital Ranged",
                            utils::WeaponType::LightHammer => "Light Hammer",
                            utils::WeaponType::LightCrossbow => "Light Crossbow",
                            utils::WeaponType::WarPick => "War Pick",
                            utils::WeaponType::HandCrossbow => "Hand Crossbow",
                            utils::WeaponType::HeavyCrossbow => "Heavy Crossbow",
                            _ => &format!("{:?}", weapon),
                        };
                        let radio = widget::radio(
                            label,
                            weapon,
                            self.proficiency_radios.weapon_type,
                            Message::ProficiencyWeaponTypeSelected,
                        );
                        if i % 2 == 0 || i == num_weapons {
                            weapon_radios2 = weapon_radios2.push(radio);
                        } else {
                            weapon_radios = weapon_radios.push(radio);
                        }
                    }
                    inner = inner.push(weapon_radios);
                    inner = inner.push(weapon_radios2);
                }

                utils::ProficiencyType::Tools(_) => todo!(),
                utils::ProficiencyType::SavingThrows(_) => todo!(),
                utils::ProficiencyType::Skills(_) => todo!(),
            }
        }

        // TODO: Add input filed for extra context

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

    /// Returns a list of all proficiency levels.
    fn all_proficiency_levels() -> Vec<utils::ProficiencyLevel> {
        vec![
            utils::ProficiencyLevel::Proficient,
            utils::ProficiencyLevel::Expertise,
        ]
    }

    /// Returns a list of all proficiency types.
    fn all_proficiency_types() -> Vec<utils::ProficiencyType> {
        vec![
            utils::ProficiencyType::Armor(utils::ArmorType::Light),
            utils::ProficiencyType::Weapons(utils::WeaponType::SimpleMelee),
            utils::ProficiencyType::Tools(utils::ToolType::DisguiseKit),
            utils::ProficiencyType::SavingThrows(utils::Attribute::Any),
            utils::ProficiencyType::Skills(utils::Skills::Arcana),
        ]
    }

    /// Returns a list of all armor types.
    fn all_armor_types() -> Vec<utils::ArmorType> {
        vec![
            utils::ArmorType::Light,
            utils::ArmorType::Medium,
            utils::ArmorType::Heavy,
            utils::ArmorType::Shield,
        ]
    }

    /// Returns a list of all weapon types.
    fn all_weapon_types() -> Vec<utils::WeaponType> {
        vec![
            utils::WeaponType::SimpleMelee,
            utils::WeaponType::SimpleRanged,
            utils::WeaponType::MartialMelee,
            utils::WeaponType::MaritalRanged,
            utils::WeaponType::Club,
            utils::WeaponType::Dagger,
            utils::WeaponType::Greatclub,
            utils::WeaponType::Handaxe,
            utils::WeaponType::Javelin,
            utils::WeaponType::LightHammer,
            utils::WeaponType::Mace,
            utils::WeaponType::Quarterstaff,
            utils::WeaponType::Sickle,
            utils::WeaponType::Spear,
            utils::WeaponType::LightCrossbow,
            utils::WeaponType::Dart,
            utils::WeaponType::Shortbow,
            utils::WeaponType::Sling,
            utils::WeaponType::Battleaxe,
            utils::WeaponType::Flail,
            utils::WeaponType::Glaive,
            utils::WeaponType::Greataxe,
            utils::WeaponType::Greatsword,
            utils::WeaponType::Halberd,
            utils::WeaponType::Lance,
            utils::WeaponType::Longsword,
            utils::WeaponType::Maul,
            utils::WeaponType::Morningstar,
            utils::WeaponType::Pike,
            utils::WeaponType::Rapier,
            utils::WeaponType::Scimitar,
            utils::WeaponType::Shortsword,
            utils::WeaponType::Trident,
            utils::WeaponType::WarPick,
            utils::WeaponType::Warhammer,
            utils::WeaponType::Whip,
            utils::WeaponType::Blowgun,
            utils::WeaponType::HandCrossbow,
            utils::WeaponType::HeavyCrossbow,
            utils::WeaponType::Longbow,
            utils::WeaponType::Net,
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

        widget::scrollable(content).width(iced::Fill).into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::VisionRadioSelected(vision) => {
                self.vision_radios.0 = Some(vision);
                self.effect = utils::TraitEffect::Vision(vision);
                Action::None
            }
            Message::VisionInputEdit(value) => {
                if let Some(vision) = &mut self.vision_radios.0 {
                    vision.set_value(value.parse().unwrap_or_default());
                }
                Action::None
            }
            Message::SavingThrowAdvantageSelected(advantage) => {
                self.saving_throw_radios.advantage = Some(advantage);
                Action::None
            }
            Message::SavingThrowTypeSelected(saving_throw_type) => {
                self.saving_throw_radios.kind = Some(saving_throw_type);
                Action::None
            }
            Message::SavingThrowAttributeSelected(attribute) => {
                self.saving_throw_radios.attribute = Some(attribute);
                Action::None
            }
            Message::SavingThrowDamageSelected(damage_type) => {
                self.saving_throw_radios.damage_type = Some(damage_type);
                Action::None
            }
            Message::ResistanceTypeSelected(resistance) => {
                self.resistance_radios.kind = Some(resistance);
                Action::None
            }
            Message::ResistanceDamageSelected(damage_type) => {
                self.resistance_radios.damage_type = Some(damage_type);
                Action::None
            }
            Message::ProficiencyLevelSelected(proficiency_level) => {
                self.proficiency_radios.level = Some(proficiency_level);
                Action::None
            }
            Message::ProficiencyTypeSelected(proficiency_type) => {
                self.proficiency_radios.kind = Some(proficiency_type);
                Action::None
            }
            Message::ProficiencyArmorTypeSelected(armor_type) => {
                self.proficiency_radios.armor_type = Some(armor_type);
                Action::None
            }
            Message::ProficiencyWeaponTypeSelected(weapon_type) => {
                self.proficiency_radios.weapon_type = Some(weapon_type);
                Action::None
            }
            Message::BackButtonPressed => Action::Cancel,
            Message::CreateButtonPressed => Action::Create(self.effect.clone()),
        }
    }
}
