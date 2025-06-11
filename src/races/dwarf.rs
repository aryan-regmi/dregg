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
    let main = String::from("Kingdoms rich in ancient grandeur, halls carved into the roots of mountains, the echoing of picks and hammers in deep mines and blazing forges, a commitment to clan and tradition, and a burning hatred of goblins and orcs — these common threads unite all dwarves.");
    let mut subsections = Vec::with_capacity(5);

    subsections.push((
        "Short and Stout".into(),
        "Bold and hardy, dwarves are known as skilled warriors, miners, and workers of stone and metal. Though they stand well under 5 feet tall, dwarves are so broad and compact that they can weigh as much as a human standing nearly two feet taller. Their courage and endurance are also easily a match for any of the larger folk.\n\nDwarven skin ranges from deep brown to a paler hue tinged with red, but the most common shades are light brown or deep tan, like certain tones of earth. Their hair, worn long but in simple styles, is usually black, gray, or brown, though paler dwarves often have red hair. Male dwarves value their beards highly and groom them carefully.".into()
    ));

    subsections.push((
        "Long Memory, Long Grudges".into(),
        "Dwarves can live to be more than 400 years old, so the oldest living dwarves often remember a very different world. For example, some of the oldest dwarves living in Citadel Felbarr (in the world of the Forgotten Realms) can recall the day, more than three centuries ago, when orcs conquered the fortress and drove them into an exile that lasted over 250 years. This longevity grants them a perspective on the world that shorter-lived races such as humans and halflings lack.\n\nDwarves are solid and enduring like the mountains they love, weathering the passage of centuries with stoic endurance and little change. They respect the traditions of their clans, tracing their ancestry back to the founding of their most ancient strongholds in the youth of the world, and don’t abandon those traditions lightly. Part of those traditions is devotion to the gods of the dwarves, who uphold the dwarven ideals of industrious labor, skill in battle, and devotion to the forge.\n\nIndividual dwarves are determined and loyal, true to their word and decisive in action, sometimes to the point of stubbornness. Many dwarves have a strong sense of justice, and they are slow to forget wrongs they have suffered. A wrong done to one dwarf is a wrong done to the dwarf’s entire clan, so what begins as one dwarf’s hunt for vengeance can become a full-blown clan feud.".into()
    ));

    subsections.push((
        "Clans and Kingdoms".into(),
        "Dwarven kingdoms stretch deep beneath the mountains where the dwarves mine gems and precious metals and forge items of wonder. They love the beauty and artistry of precious metals and fine jewelry, and in some dwarves this love festers into avarice. Whatever wealth they can’t find in their mountains, they gain through trade. They dislike boats, so enterprising humans and halflings frequently handle trade in dwarven goods along water routes. Trustworthy members of other races are welcome in dwarf settlements, though some areas are off limits even to them.\n\nThe chief unit of dwarven society is the clan, and dwarves highly value social standing. Even dwarves who live far from their own kingdoms cherish their clan identities and affiliations, recognize related dwarves, and invoke their ancestors’ names in oaths and curses. To be clanless is the worst fate that can befall a dwarf.\n\nDwarves in other lands are typically artisans, especially weaponsmiths, armorers, and jewelers. Some become mercenaries or bodyguards, highly sought after for their courage and loyalty.".into()
    ));

    subsections.push((
        "Gods, Gold, and Clan".into(),
        "Dwarves who take up the adventuring life might be motivated by a desire for treasure — for its own sake, for a specific purpose, or even out of an altruistic desire to help others. Other dwarves are driven by the command or inspiration of a deity, a direct calling or simply a desire to bring glory to one of the dwarf gods. Clan and ancestry are also important motivators. A dwarf might seek to restore a clan’s lost honor, avenge an ancient wrong the clan suffered, or earn a new place within the clan after having been exiled. Or a dwarf might search for the axe wielded by a mighty ancestor, lost on the field of battle centuries ago.".into()
    ));

    subsections.push((
        "Dwarf Names".into(),
        "A dwarf’s name is granted by a clan elder, in accordance with tradition. Every proper dwarven name has been used and reused down through the generations. A dwarf’s name belongs to the clan, not to the individual. A dwarf who misuses or brings shame to a clan name is stripped of the name and forbidden by law to use any dwarven name in its place.\n\nMale Names: Adrik, Alberich, Baern, Barendd, Brottor, Bruenor, Dain, Darrak, Delg, Eberk, Einkil, Fargrim, Flint, Gardain, Harbek, Kildrak, Morgran, Orsik, Oskar, Rangrim, Rurik, Taklinn, Thoradin, Thorin, Tordek, Traubon, Travok, Ulfgar, Veit, Vondal\n\nFemale Names: Amber, Artin, Audhild, Bardryn, Dagnal, Diesa, Eldeth, Falkrunn, Finellen, Gunnloda, Gurdis, Helja, Hlin, Kathra, Kristryd, Ilde, Liftrasa, Mardred, Riswynn, Sannl, Torbera, Torgga, Vistra\n\nClan Names: Balderk, Battlehammer, Brawnanvil, Dankil, Fireforge, Frostbeard, Gorunn, Holderhek, Ironfist, Loderr, Lutgehr, Rumnaheim, Strakeln, Torunn, Ungart".into()
    ));

    Summary { main, subsections }
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
                tags: vec!["armor", "proficiency"].iter().map(|s| String::from(*s)).collect(),
            }]),
        }
}

}
