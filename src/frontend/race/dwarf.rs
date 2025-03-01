use super::{
    Action, Age, Attribute, Choices, Race, RacialTrait, Size, SizeCategory, Speed, Subrace,
};

pub fn dwarf() -> Race {
    Race {
        name: "Dwarf".into(),
        plural_name: "Dwarves".into(),
        summary: "Bold and hardy, dwarves are known as skilled warriors, miners, and workers of stone and metal. Though they stand well under 5 feet tall, dwarves are so broad and compact that they can weigh as much as a human standing nearly two feet taller. Their courage and endurance are also easily a match for any of the larger folk.\nDwarven skin ranges from deep brown to a paler hue tinged with red, but the most common shades are light brown or deep tan, like certain tones of earth. Their hair, worn long but in simple styles, is usually black, gray, or brown, though paler dwarves often have red hair. Male dwarves value their beards highly and groom them carefully".into(),
        asi: vec![Attribute::Constitution(2)],
        age: Age {
            adult: 50,
            lifespan: 350,
        },
        size: Size {
            size: SizeCategory::Medium,
            height: Some(4.0),
            weight: Some(150.0),
        },
        speed: vec![Speed::Walking(25)],
        languages: vec!["Common", "Dwarvish"]
            .into_iter()
            .map(String::from)
            .collect(),
        proficiencies: vec![Choices::One(
            vec!["Smith's Tools", "Brewer's Supplies", "Mason's Tools"]
                .into_iter()
                .map(String::from)
                .collect(),
        )],
        subraces: vec![hill_dwarf(), mountain_dwarf()],
        traits: vec![DWARVEN_RESILIENCE, DARKVISION, DWARVEN_SPEED, STONECUNNING],
    }
}

pub fn hill_dwarf() -> Subrace {
    Subrace {
        name: "Hill Dwarf".into(),
        summary: "As a hill dwarf, you have keen senses, deep intuition, and remarkable resilience. The gold dwarves of Faerûn in their mighty southern kingdom are hill dwarves, as are the exiled Neidar and the debased Klar of Krynn in the Dragonlance setting.".into(),
        asi: vec![Attribute::Wisdom(1)],
        languages: None,
        proficiencies: vec![],
        traits: vec![DWARVEN_TOUGHNESS],
    }
}

pub fn mountain_dwarf() -> Subrace {
    Subrace {
        name: "Mountain Dwarf".into(),
        summary: "As a mountain dwarf, you’re strong and hardy, accustomed to a difficult life in rugged terrain. You’re probably on the tall side (for a dwarf), and tend toward lighter coloration. The shield dwarves of northern Faerûn, as well as the ruling Hylar clan and the noble Daewar clan of Dragonlance, are mountain dwarves.".into(),
        asi: vec![Attribute::Strength(2)],
        languages: None,
        proficiencies: vec![],
        traits: vec![DWARVEN_ARMOR_TRAINING],
    }
}

const DWARVEN_RESILIENCE: RacialTrait = RacialTrait {
    name: "Dwarven Resilience",
    summary: "You have advantage on saving throws against poison, and you have resistance against poison damage.",
    action_type: Action::None,
};

// TODO: Move to common traits?
const DARKVISION: RacialTrait = RacialTrait {
    name: "Darkvision",
    summary: "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.",
    action_type: Action::None,
};

const DWARVEN_TOUGHNESS: RacialTrait = RacialTrait {
    name: "Dwarven Toughness",
    summary:
        "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.",
    action_type: Action::None,
};

const DWARVEN_ARMOR_TRAINING: RacialTrait = RacialTrait {
    name: "Dwarven Armor Training",
    summary: "You have proficiency with light and medium armor.",
    action_type: Action::None,
};

const DWARVEN_SPEED: RacialTrait = RacialTrait {
    name: "Dwarven Speed",
    summary: "Your speed is not reduced by wearing heavy armor.",
    action_type: Action::None,
};

const STONECUNNING: RacialTrait = RacialTrait {
    name: "Stonecunning",
    summary: "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.",
    action_type: Action::None,
};
