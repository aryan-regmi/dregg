use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    pub name: String,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
