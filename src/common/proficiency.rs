use super::{Attribute, Skills};

/// Represents a proficiency.
#[derive(Debug)]
pub struct Proficiency {
    /// The proficiency level (Proficient or Expertise).
    pub level: ProficiencyLevel,

    /// Determines what the proficiency applies to.
    pub kind: ProficiencyType,

    /// Extra context that is required for proficiencies with conditional activations.
    pub context: Option<String>,
}

/// Represents a level of proficiency.
#[derive(Debug, Default)]
pub enum ProficiencyLevel {
    #[default]
    Proficient,
    Expertise,
}

/// Represents a type of proficiency.
#[derive(Debug)]
pub enum ProficiencyType {
    Armor(ArmorType),
    Weapons(WeaponType),
    Tools(ToolType),
    SavingThrows(Attribute),
    Skills(Skills),
}

/// Represents types of armors.
#[derive(Debug)]
pub enum ArmorType {
    /// Light armor.
    Light,

    /// Medium armor.
    Medium,

    /// Heavy armor.
    Heavy,

    /// Shields.
    Shield,
}

/// Represents types of weapons.
#[derive(Debug)]
pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MaritalRanged,
    Club,
    Dagger,
    Greatclub,
    Handaxe,
    Javelin,
    LightHammer,
    Mace,
    Quarterstaff,
    Sickle,
    Spear,
    LightCrossbow,
    Dart,
    Shortbow,
    Sling,
    Battleaxe,
    Flail,
    Glaive,
    Greataxe,
    Greatsword,
    Halberd,
    Lance,
    Longsword,
    Maul,
    Morningstar,
    Pike,
    Rapier,
    Scimitar,
    Shortsword,
    Trident,
    WarPick,
    Warhammer,
    Whip,
    Blowgun,
    HandCrossbow,
    HeavyCrossbow,
    Longbow,
    Net,
}

/// Represents types of tools.
#[derive(Debug)]
pub enum ToolType {
    ArtisansTools(ArtisansTools),
    DisguiseKit,
    ForgeryKit,
    GamingSet(GamingSet),
    HerbalismKit,
    MusicalInstrument(MusicalInstrument),
    NavigatorsTools,
    PoisonersKit,
    ThievesTools,
}

/// Represents artisan tools.
#[derive(Debug)]
pub enum ArtisansTools {
    AlchemistsSupplies,
    BrewersSupplies,
    CalligraphersSupplies,
    CarpentersTools,
    CartographersTools,
    CobblersTools,
    CooksUtensils,
    GlassblowersTools,
    JewelersTools,
    LeatherworkersTools,
    MasonsTools,
    PaintersSupplies,
    PottersTools,
    SmithsTools,
    TinkersTools,
    WeaversTools,
    WoodcarversTools,
}

/// Represents gaming sets.
#[derive(Debug)]
pub enum GamingSet {
    DiceSet,
    DragonchessSet,
    PlayingCardSet,
    ThreeDragonAnteSet,
}

/// Represents musical instruments.
#[derive(Debug)]
pub enum MusicalInstrument {
    Bagpipes,
    Drum,
    Dulcimer,
    Flute,
    Lute,
    Lyre,
    Horn,
    PanFlute,
    Shawm,
    Viol,
}
