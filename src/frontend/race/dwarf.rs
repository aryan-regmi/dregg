use super::{Action, Age, Attribute, Choices, Race, RacialTrait, Size, Subrace};

pub fn dwarf() -> Race {
    Race {
        name: "Dwarf".into(),
        summary: "TODO: Need to fill in summary!".into(),
        asi: vec![Attribute::Constitution(2)],
        age: Age {
            adult: 50,
            lifespan: 350,
        },
        size: Size::Medium,
        speed: 25,
        languages: vec!["Common", "Dwarvish"]
            .into_iter()
            .map(String::from)
            .collect(),
        proficiencies: Choices::One(
            vec!["Smith's Tools", "Brewer's Supplies", "Mason's Tools"]
                .into_iter()
                .map(String::from)
                .collect(),
        ),
        subraces: vec![hill_dwarf()],
        traits: vec![DWARVEN_RESILIENCE, DARKVISION],
    }
}

fn hill_dwarf() -> Subrace {
    Subrace {
        name: "Hill Dwarf".into(),
        summary: "TODO: Need to fill summary!".into(),
        asi: vec![Attribute::Wisdom(1)],
        languages: None,
        proficiencies: None,
        traits: vec![],
    }
}

fn mountain_dwarf() -> Subrace {
    Subrace {
        name: "Mountain Dwarf".into(),
        summary: "TODO: Need to fill summary!".into(),
        asi: vec![Attribute::Strength(2)],
        languages: None,
        proficiencies: None,
        traits: vec![],
    }
}

const DWARVEN_RESILIENCE: RacialTrait = RacialTrait {
    name: "Dwarven Resilience",
    summary: "TODO: Need to fill summary!",
    action_type: Action::None,
};

// TODO: Move to common traits?
const DARKVISION: RacialTrait = RacialTrait {
    name: "Darkvision",
    summary: "TODO: Need to fill summary!",
    action_type: Action::None,
};

const DWARVEN_TOUGHNESS: RacialTrait = RacialTrait {
    name: "Dwarven Toughness",
    summary: "TODO: Need to fill summary!",
    action_type: Action::None,
};

const DWARVEN_ARMOR_TRAINING: RacialTrait = RacialTrait {
    name: "Dwarven Armor Training",
    summary: "TODO: Need to fill summary!",
    action_type: Action::None,
};
