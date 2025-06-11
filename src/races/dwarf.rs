use crate::{
    race::Race,
    utils::{
         darkvision, Age, AgeInfo, Attribute, Height, Language, LanguageLevel, Range, RangeTrait, Size, SizeInfo, Speed, Summary, Weight, ASI
    },
};

pub fn dwarf() -> Race {
    Race {
        name: "Dwarf".into(),
        plural_name: Some("Dwarves".into()),
        summary: summary(),
        asi: Some(vec![ASI {
            attribute: Attribute::Constitution,
            value: 2,
        }]),
        age: Some(AgeInfo {
            adult: Age(50),
            lifespan: Age(300),
        }),
        size: SizeInfo {
            category: Size::Medium,
            height: Some(Range {
                start: Height {
                    feet: 4.0,
                    inches: 0.0,
                },
                end: Height {
                    feet: 5.0,
                    inches: 0.0,
                },
            }),
            weight: Some(Range::singular(Weight(150.0))),
        },
        speed: vec![Speed::Walking(25)],
        traits: Some(vec![
            darkvision(60),
            traits::dwarven_speed(),
            traits::dwarven_resilience(),
            traits::dwarven_combat_training(),
            traits::tool_proficiency(),
            traits::stonecunning(),
        ]),
        languages: Some(languages()),
        subraces: Some(vec![
            subraces::hill_dwarf(),
            subraces::mountain_dwarf(),
        ]),
    }
}

fn summary() -> Summary {
    Summary {
        main: "Summary Here".into(),
        subsections: vec![("H1".into(), "Paragraph".into())],
    }
}

fn languages() -> Vec<Language> {
    vec![
        Language {
            name: "Common".into(),
            levels: vec![
                LanguageLevel::Speak,
                LanguageLevel::Read,
                LanguageLevel::Write,
            ],
        },
        Language {
            name: "Dwarvish".into(),
            levels: vec![
                LanguageLevel::Speak,
                LanguageLevel::Read,
                LanguageLevel::Write,
            ],
        },
    ]
}

mod traits {
    use crate::utils::{Advantage, ArtisansTools, Choice, DamageType, Proficiency, ProficiencyLevel, ProficiencyType, SavingThrowsType, Skills, ToolType, Trait, TraitEffect, WeaponType};

pub(super) fn dwarven_speed() -> Trait {
    Trait {
        name: "Dwarven Speed".into(),
        summary: "Your speed is not reduced by wearing heavy armor.".into(),
        effects: vec![TraitEffect::NoSpeedReduction],
        required_level: None,
        tags: vec!["speed", "dwarf", "heavy armor"]
            .iter()
            .map(|s| String::from(*s))
            .collect(),
    }
}

pub(super) fn dwarven_resilience() -> Trait {
    Trait {
        name: "Dwarven Resilience".into(),
        summary: "You have advantage on saving throws against poison, and you have resistance against poison damage.".into(),
        effects: vec![
            TraitEffect::SavingThrows {
                advantage:Advantage::Advantage,
                kind: SavingThrowsType::Damage(DamageType::Poison) 
            }
        ],
        required_level: None,
        tags: vec!["resistance", "advantage", "poison"].iter().map(|s| String::from(*s)).collect(),
    }
}

pub(super)   fn dwarven_combat_training() -> Trait {
    Trait {
        name: "Dwarven Combat Training".into(),
        summary: "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer."
            .into(),
        effects: vec![TraitEffect::Proficiencies(Choice::AllOf(vec![
            Proficiency {
                level: ProficiencyLevel::Proficient,
                kind: ProficiencyType::Weapons(WeaponType::Battleaxe),
                context: None,
            },
            Proficiency {
                level: ProficiencyLevel::Proficient,
                kind: ProficiencyType::Weapons(WeaponType::LightHammer),
                context: None,
            },
            Proficiency {
                level: ProficiencyLevel::Proficient,
                kind: ProficiencyType::Weapons(WeaponType::Warhammer),
                context: None,
            },
        ]))],
        required_level: None,
        tags: vec!["proficiency", "weapons"]
            .iter()
            .map(|s| String::from(*s))
            .collect(),
    }
}

pub(super)  fn tool_proficiency() -> Trait {
    Trait {
        name: "Tool Proficiency".into(),
        summary: "You gain proficiency with the artisan’s tools of your choice: smith’s tools, brewer’s supplies, or mason’s tools.".into(),
        effects: vec![
            TraitEffect::Proficiencies(Choice::OneOf(vec![
                    Proficiency { 
                        level: ProficiencyLevel::Proficient,
                        kind: ProficiencyType::Tools(ToolType::ArtisansTools(ArtisansTools::SmithsTools)),
                        context: None 
                    },
                    Proficiency { 
                        level: ProficiencyLevel::Proficient,
                        kind: ProficiencyType::Tools(ToolType::ArtisansTools(ArtisansTools::BrewersSupplies)),
                        context: None 
                    },
                    Proficiency { 
                        level: ProficiencyLevel::Proficient,
                        kind: ProficiencyType::Tools(ToolType::ArtisansTools(ArtisansTools::MasonsTools)),
                        context: None 
                    },
            ]))
        ],
        required_level: None,
        tags: vec!["proficiency", "tools"].iter().map(|s| String::from(*s)).collect(),
    }
}

pub(super)  fn stonecunning() -> Trait {
    Trait {
        name: "Stonecunning".into(),
        summary: "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus".into(),
        effects: vec![
            TraitEffect::Proficiencies(Choice::Single(Proficiency { 
                level: ProficiencyLevel::Expertise,
                kind: ProficiencyType::Skills(Skills::History),
                context: Some("Related to the origin of stonework".into()) 
            }))
        ],
        required_level: None,
        tags: vec!["proficiency", "history", "intelligence", "history"].iter().map(|s| String::from(*s)).collect(),
    }
}

}

mod subraces {
    use crate::{race::Subrace, utils::{ArmorType, Attribute, Choice, HpIncrease, Proficiency, ProficiencyLevel, ProficiencyType, Summary, Trait, TraitEffect, ASI}};

    pub fn hill_dwarf() -> Subrace {
        let summary = Summary { 
            main: "As a hill dwarf, you have keen senses, deep intuition, and remarkable resilience. The gold dwarves of Faerûn in their mighty southern kingdom are hill dwarves, as are the exiled Neidar and the debased Klar of Krynn in the Dragonlance setting.".into(),
            subsections: vec![],
        };
        
        Subrace{
            name: "Hill Dwarf".into(),
            summary,
            asi: Some(vec![ASI {
                attribute: Attribute::Wisdom,
                value: 1,
            }]),
            languages: None,
            traits: Some(vec![Trait { 
                name: "Dwarven Toughness".into(),
                summary: "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.".into(),
                effects: vec![TraitEffect::HpIncrease(HpIncrease::Max(1)), TraitEffect::HpIncrease(HpIncrease::PerLevel(1))],
                required_level: None,
                tags: vec!["hp", "hit points"].iter().map(|s| String::from(*s)).collect(),
            }]),
        }
}

    pub fn mountain_dwarf() -> Subrace {
        let summary = Summary { 
            main: "As a mountain dwarf, you’re strong and hardy, accustomed to a difficult life in rugged terrain. You’re probably on the tall side (for a dwarf), and tend toward lighter coloration. The shield dwarves of northern Faerûn, as well as the ruling Hylar clan and the noble Daewar clan of Dragonlance, are mountain dwarves.s a hill dwarf, you have keen senses, deep intuition, and remarkable resilience. The gold dwarves of Faerûn in their mighty southern kingdom are hill dwarves, as are the exiled Neidar and the debased Klar of Krynn in the Dragonlance setting.".into(),
            subsections: vec![],
        };
        
        Subrace {
            name: "Mountain Dwarf".into(),
            summary,
            // asi: Some(vec![ASI::Strength(2)]),
            asi: Some(vec![ASI {
                attribute: Attribute::Strength,
                value: 2, 
            }]),
            languages: None,
            traits: Some(vec![Trait { 
                name: "Dwarven Armor Training".into(),
                summary: "You have proficiency with light and medium armor.".into(),
                effects: vec![TraitEffect::Proficiencies(Choice::AllOf(vec![
                        Proficiency { 
                            level: ProficiencyLevel::Proficient,
                            kind: ProficiencyType::Armor(ArmorType::Light),
                            context: None,
                        },
                        Proficiency { 
                            level: ProficiencyLevel::Proficient,
                            kind: ProficiencyType::Armor(ArmorType::Medium),
                            context: None,
                        },
                ]))],
                required_level: None,
                tags: vec!["hp", "hit points"].iter().map(|s| String::from(*s)).collect(),
            }]),
        }
}

}
