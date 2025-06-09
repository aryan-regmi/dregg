use crate::{
    race::Race,
    utils::{
        self, darkvision, Advantage, Age, AgeInfo, ArtisansTools, Attribute, Choice, DamageType, Height, Language, LanguageLevel, Proficiency, ProficiencyLevel, ProficiencyType, Range, RangeTrait, SavingThrowsType, Size, SizeInfo, Skills, Speed, Summary, ToolType, Trait, TraitEffect, WeaponType, Weight, ASI
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
        traits: vec![
            darkvision(60),
            dwarven_speed(),
            dwarven_resilience(),
            dwarven_combat_training(),
            tool_proficiency(),
            stonecunning(),
        ],
        languages: Some(languages()),
    }
}

fn summary() -> Summary {
    Summary {
        main: "Summary Here".into(),
        subsections: vec![("H1".into(), "Paragraph".into())],
        base_padding: utils::styles::BASE_PADDING,
        summary_padding: utils::styles::SUMMARY_PADDING,
        summary_subsection_padding: utils::styles::SUMMARY_SUBSECTION_PADDING,
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

fn dwarven_speed() -> Trait {
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

fn dwarven_resilience() -> Trait {
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

fn dwarven_combat_training() -> Trait {
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

fn tool_proficiency() -> Trait {
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

fn stonecunning() -> Trait {
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
