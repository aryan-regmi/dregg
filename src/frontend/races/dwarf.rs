use std::collections::HashMap;

use crate::frontend::{
    race::{
        Age, Attribute, Choices, Height, Language, LanguageLevel, Race, RacialTrait, Range, Size,
        SizeCategory, Speed, Subrace, Summary,
    },
    races::common::darkvision,
};

pub fn dwarf() -> Race {
    Race {
        name: "Dwarf".into(),
        name_plural: "Dwarves".into(),
        summary: summary(),
        asi: vec![Attribute::Constitution(2)],
        age: Age {
            adult: 50,
            lifespan: 350,
        },
        size: size(),
        speed: vec![Speed::Walking(25)],
        languages: languages(),
        proficiencies: proficiencies(),
        subraces: vec![hill_dwarf(), mountain_dwarf()],
        traits: traits(),
    }
}

fn summary() -> Summary {
    let main = String::from("Kingdoms rich in ancient grandeur, halls carved into the roots of mountains, the echoing of picks and hammers in deep mines and blazing forges, a commitment to clan and tradition, and a burning hatred of goblins and orcs — these common threads unite all dwarves.");
    let mut subsections = HashMap::new();

    subsections.insert(
        "Short and Stout".into(),
        "Bold and hardy, dwarves are known as skilled warriors, miners, and workers of stone and metal. Though they stand well under 5 feet tall, dwarves are so broad and compact that they can weigh as much as a human standing nearly two feet taller. Their courage and endurance are also easily a match for any of the larger folk.\n\n
        Dwarven skin ranges from deep brown to a paler hue tinged with red, but the most common shades are light brown or deep tan, like certain tones of earth. Their hair, worn long but in simple styles, is usually black, gray, or brown, though paler dwarves often have red hair. Male dwarves value their beards highly and groom them carefully.".into()
    );

    subsections.insert(
        "Long Memory, Long Grudges".into(),
        "Dwarves can live to be more than 400 years old, so the oldest living dwarves often remember a very different world. For example, some of the oldest dwarves living in Citadel Felbarr (in the world of the Forgotten Realms) can recall the day, more than three centuries ago, when orcs conquered the fortress and drove them into an exile that lasted over 250 years. This longevity grants them a perspective on the world that shorter-lived races such as humans and halflings lack.\n\n
        Dwarves are solid and enduring like the mountains they love, weathering the passage of centuries with stoic endurance and little change. They respect the traditions of their clans, tracing their ancestry back to the founding of their most ancient strongholds in the youth of the world, and don’t abandon those traditions lightly. Part of those traditions is devotion to the gods of the dwarves, who uphold the dwarven ideals of industrious labor, skill in battle, and devotion to the forge.\n\n
        Individual dwarves are determined and loyal, true to their word and decisive in action, sometimes to the point of stubbornness. Many dwarves have a strong sense of justice, and they are slow to forget wrongs they have suffered. A wrong done to one dwarf is a wrong done to the dwarf’s entire clan, so what begins as one dwarf’s hunt for vengeance can become a full-blown clan feud.".into()
    );

    subsections.insert(
        "Clans and Kingdoms".into(),
        "Dwarven kingdoms stretch deep beneath the mountains where the dwarves mine gems and precious metals and forge items of wonder. They love the beauty and artistry of precious metals and fine jewelry, and in some dwarves this love festers into avarice. Whatever wealth they can’t find in their mountains, they gain through trade. They dislike boats, so enterprising humans and halflings frequently handle trade in dwarven goods along water routes. Trustworthy members of other races are welcome in dwarf settlements, though some areas are off limits even to them.\n\n
        The chief unit of dwarven society is the clan, and dwarves highly value social standing. Even dwarves who live far from their own kingdoms cherish their clan identities and affiliations, recognize related dwarves, and invoke their ancestors’ names in oaths and curses. To be clanless is the worst fate that can befall a dwarf.\n\n
        Dwarves in other lands are typically artisans, especially weaponsmiths, armorers, and jewelers. Some become mercenaries or bodyguards, highly sought after for their courage and loyalty.".into()
    );

    subsections.insert(
        "Gods, Gold, and Clan".into(),
        "Dwarves who take up the adventuring life might be motivated by a desire for treasure — for its own sake, for a specific purpose, or even out of an altruistic desire to help others. Other dwarves are driven by the command or inspiration of a deity, a direct calling or simply a desire to bring glory to one of the dwarf gods. Clan and ancestry are also important motivators. A dwarf might seek to restore a clan’s lost honor, avenge an ancient wrong the clan suffered, or earn a new place within the clan after having been exiled. Or a dwarf might search for the axe wielded by a mighty ancestor, lost on the field of battle centuries ago.".into()
    );

    subsections.insert(
        "Dwarf Names".into(),
        "A dwarf’s name is granted by a clan elder, in accordance with tradition. Every proper dwarven name has been used and reused down through the generations. A dwarf’s name belongs to the clan, not to the individual. A dwarf who misuses or brings shame to a clan name is stripped of the name and forbidden by law to use any dwarven name in its place.\n\n
        Male Names: Adrik, Alberich, Baern, Barendd, Brottor, Bruenor, Dain, Darrak, Delg, Eberk, Einkil, Fargrim, Flint, Gardain, Harbek, Kildrak, Morgran, Orsik, Oskar, Rangrim, Rurik, Taklinn, Thoradin, Thorin, Tordek, Traubon, Travok, Ulfgar, Veit, Vondal\n\n
        Female Names: Amber, Artin, Audhild, Bardryn, Dagnal, Diesa, Eldeth, Falkrunn, Finellen, Gunnloda, Gurdis, Helja, Hlin, Kathra, Kristryd, Ilde, Liftrasa, Mardred, Riswynn, Sannl, Torbera, Torgga, Vistra\n\n
        Clan Names: Balderk, Battlehammer, Brawnanvil, Dankil, Fireforge, Frostbeard, Gorunn, Holderhek, Ironfist, Loderr, Lutgehr, Rumnaheim, Strakeln, Torunn, Ungart".into()
    );

    Summary { main, subsections }
}

fn size() -> Size {
    Size {
        category: SizeCategory::Medium,
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
        weight: Some(Range {
            start: 150.0,
            end: 150.0,
        }),
    }
}

fn languages() -> Vec<Language> {
    use LanguageLevel::*;
    vec![
        Language {
            name: "Common".into(),
            levels: vec![Speak, Read, Write],
        },
        Language {
            name: "Dwarvish".into(),
            levels: vec![Speak, Read, Write],
        },
    ]
}

fn proficiencies() -> Vec<Choices> {
    vec![Choices::One(vec![
        "Smith's tools".into(),
        "Brewer's supplies".into(),
        "Mason's tools".into(),
    ])]
}

fn hill_dwarf() -> Subrace {
    let summary = Summary {
        main: "As a hill dwarf, you have keen senses, deep intuition, and remarkable resilience. The gold dwarves of Faerûn in their mighty southern kingdom are hill dwarves, as are the exiled Neidar and the debased Klar of Krynn in the Dragonlance setting.".into(),
        subsections: HashMap::new(),
    };

    Subrace {
        name: "Hill Dwarf".into(),
        summary,
        asi: vec![Attribute::Wisdom(1)],
        languages: vec![],
        proficiencies: vec![],
        traits: vec![RacialTrait {
            name: "Dwarven Toughness".into(),
            summary: "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.".into(),
            action_type: None,
        }],
    }
}

fn mountain_dwarf() -> Subrace {
    let summary = Summary {
        main: "As a mountain dwarf, you’re strong and hardy, accustomed to a difficult life in rugged terrain. You’re probably on the tall side (for a dwarf), and tend toward lighter coloration. The shield dwarves of northern Faerûn, as well as the ruling Hylar clan and the noble Daewar clan of Dragonlance, are mountain dwarves".into(),
        subsections: HashMap::new(),
    };

    Subrace {
        name: "Mountian Dwarf".into(),
        summary,
        asi: vec![Attribute::Strength(2)],
        languages: vec![],
        proficiencies: vec![],
        traits: vec![RacialTrait {
            name: "Dwarven Armor Training".into(),
            summary: "You have proficiency with light and medium armor".into(),
            action_type: None,
        }],
    }
}

fn traits() -> Vec<RacialTrait> {
    let dwarven_resilience = RacialTrait {
        name: "Dwarven Resilience".into(),
        summary: "You have advantage on saving throws against poison, and you have resistance against poison damage.".into(),
        action_type: None,
    };

    let dwarven_combat_training = RacialTrait {
        name: "Dwarven Combat Training".into(),
        summary: "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer"
            .into(),
        action_type: None,
    };

    let stonecunning = RacialTrait {
        name: "Stonecunning".into(),
        summary: "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.".into(),
        action_type: None,
    };

    let dwarven_speed = RacialTrait {
        name: "Dwarven Speed".into(),
        summary: "Your speed is not reduced by wearing heavy armor.".into(),
        action_type: None,
    };

    vec![
        darkvision(),
        dwarven_speed,
        dwarven_resilience,
        dwarven_combat_training,
        stonecunning,
    ]
}
