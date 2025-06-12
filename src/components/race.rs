use std::fmt::Display;

// use iced::widget;

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Action {
    /// No action required.
    None,
}

/// A race component.
#[derive(Debug, PartialEq, Clone)]
pub struct Race {
    /// The name of the race.
    pub name: String,

    /// The plural name of the race.
    ///
    /// This is used for stringifying certain values.
    pub plural_name: String,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
