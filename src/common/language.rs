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

impl std::fmt::Display for LanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageLevel::Speak => f.write_str("Speak"),
            LanguageLevel::Read => f.write_str("Read"),
            LanguageLevel::Write => f.write_str("Write"),
            LanguageLevel::Understand => f.write_str("Understand"),
        }
    }
}
