/// Represents a language a character knows.
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: String,
    pub levels: Vec<LanguageLevel>,
}

/// Represents the various levels of proficiency in a language.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanguageLevel {
    Speak,
    Read,
    Write,
    Understand,
}
