pub mod dwarf;

#[derive(Debug, Clone, PartialEq)]
pub enum Choices {
    /// A list of choices out of which only one can be selected.
    One(Vec<String>),

    /// A list all choices a character gets.
    All(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    /// Name of the race.
    pub name: String,

    /// The summary/description of the race.
    pub summary: String,

    /// Ability score increases provided by the race.
    pub asi: Vec<Attribute>,

    /// The age info of the race.
    pub age: Age,

    /// The size of the race.
    pub size: Size,

    /// The speed of the race.
    pub speed: u16,

    /// The various languages that a character of the race can speak.
    pub languages: Vec<String>,

    /// The proficiencies the race provides.
    pub proficiencies: Choices,

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
    adult: u16,

    /// The average/expected lifespan of a character of the race.
    lifespan: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RacialTrait {
    /// The name of an action.
    name: &'static str,

    /// The summary/description of the action.
    summary: &'static str,

    /// The type of action (`Action`, `Bonus Action`, or `Reaction`).
    action_type: Action,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
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
