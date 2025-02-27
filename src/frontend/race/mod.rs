use iced::Element;

pub mod dwarf;

#[derive(Debug, Clone, PartialEq)]
pub enum Choices {
    /// A list of choices out of which only one can be selected.
    One(Vec<String>),

    /// A list all choices a character gets.
    All(Vec<String>),
}

impl Choices {
    pub fn text(&self, header: &str) -> String {
        match self {
            Choices::One(items) => format!(
                "{} one of the following of your choice: {}.",
                header,
                items.join(", ")
            ),
            Choices::All(items) => {
                format!("{} all of the following: {}.", header, items.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    /// Name of the race.
    pub name: String,

    /// The plural form of the race.
    pub plural_name: String,

    /// The summary/description of the race.
    pub summary: String,

    /// Ability score increases provided by the race.
    pub asi: Vec<Attribute>,

    /// The age info of the race.
    pub age: Age,

    /// The size of the race.
    pub size: Size,

    /// The speed of the race.
    pub speed: Vec<Speed>,

    /// The various languages that a character of the race can speak.
    pub languages: Vec<String>,

    /// The proficiencies the race provides.
    pub proficiencies: Vec<Choices>,

    /// Subraces of the race that a character may choose.
    pub subraces: Vec<Subrace>,

    /// A list of traits the race provides.
    pub traits: Vec<RacialTrait>,
}

impl Into<RaceName> for Race {
    fn into(self) -> RaceName {
        match self.name.as_str() {
            "Dwarf" => RaceName::Dwarf,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subrace {
    /// Name of the race.
    name: String,

    /// The summary/description of the race.
    summary: String,

    /// Ability score increases provided by the race.
    asi: Vec<Attribute>,

    /// The proficiencies the race provides.
    proficiencies: Option<Choices>,

    /// The various languages that a character of the race can speak.
    languages: Option<Vec<String>>,

    /// A list of traits the race provides.
    traits: Vec<RacialTrait>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Age {
    /// The age at which a character of the race is considered an adult.
    pub adult: u16,

    /// The average/expected lifespan of a character of the race.
    pub lifespan: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RacialTrait {
    /// The name of an action.
    pub name: &'static str,

    /// The summary/description of the action.
    pub summary: &'static str,

    /// The type of action (`Action`, `Bonus Action`, or `Reaction`).
    pub action_type: Action,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    None,
    Action,
    BonusAction,
    Reaction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    Strength(u8),
    Dexterity(u8),
    Constitution(u8),
    Intelligence(u8),
    Wisdom(u8),
    Charisma(u8),
}

impl Attribute {
    pub fn text(&self) -> String {
        match self {
            Attribute::Strength(amount) => format!("Strength score increases by {amount}.\n"),
            Attribute::Dexterity(amount) => format!("Dexterity score increases by {amount}.\n"),
            Attribute::Constitution(amount) => {
                format!("Constitution score increases by {amount}.\n")
            }
            Attribute::Intelligence(amount) => {
                format!("Intelligence score increases by {amount}.\n")
            }
            Attribute::Wisdom(amount) => format!("Wisdom score increases by {amount}.\n"),
            Attribute::Charisma(amount) => format!("Charisma score increases by {amount}.\n"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    /// The size category.
    pub size: SizeCategory,

    /// Height in feet.
    pub height: Option<f32>,

    /// Weight in pounds.
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
}

impl SizeCategory {
    pub fn text(&self) -> String {
        match self {
            SizeCategory::Tiny => "Tiny".into(),
            SizeCategory::Small => "Small".into(),
            SizeCategory::Medium => "Medium".into(),
            SizeCategory::Large => "Large".into(),
            SizeCategory::Gargantuan => "Gargantuan".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}

impl Speed {
    pub fn text(&self) -> String {
        match self {
            Speed::Walking(distance) => format!("Walking speed of {distance} feet."),
            Speed::Flying(distance) => format!("Flying speed of {distance} feet."),
            Speed::Swimming(distance) => format!("Swimming speed of {distance} feet."),
            Speed::Climbing(distance) => format!("Climbing speed of {distance} feet."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaceName {
    Dwarf = 0,
    Count,
}

impl RaceName {
    pub const ALL: [RaceName; RaceName::Count as usize] = [RaceName::Dwarf];

    /// Returns the list of races.
    pub fn races() -> Vec<Race> {
        vec![dwarf::dwarf()]
    }
}

impl ToString for RaceName {
    fn to_string(&self) -> String {
        match self {
            RaceName::Dwarf => "Dwarf".into(),
            RaceName::Count => unreachable!("The `Count` variant should never be used"),
        }
    }
}

impl Into<Race> for RaceName {
    fn into(self) -> Race {
        match self {
            RaceName::Dwarf => dwarf::dwarf(),
            RaceName::Count => unreachable!(),
        }
    }
}
