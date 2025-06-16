/// Represents the various types of movements.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Movement {
    #[default]
    Walking,
    Flying,
    Swimming,
    Climbing,
}

/// Represents a speed of a character.
#[derive(Debug, Default, Clone)]
pub struct Speed {
    pub(crate) movement: Movement,
    pub(crate) value: u8,
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.movement {
            Movement::Walking => f.write_str("Walking"),
            Movement::Flying => f.write_str("Flying"),
            Movement::Swimming => f.write_str("Swimming"),
            Movement::Climbing => f.write_str("Climbing"),
        }
    }
}

impl Speed {
    pub fn to_text(&self) -> String {
        match self.movement {
            Movement::Walking => format!("You have a walking speed of {} feet. ", self.value),
            Movement::Flying => format!("You have a flying speed of {} feet. ", self.value),
            Movement::Swimming => format!("You have a swimming speed of {} feet. ", self.value),
            Movement::Climbing => format!("You have a climbing speed of {} feet. ", self.value),
        }
    }
}

pub const SPEEDS: [Speed; 4] = [
    Speed {
        movement: Movement::Walking,
        value: 0,
    },
    Speed {
        movement: Movement::Flying,
        value: 0,
    },
    Speed {
        movement: Movement::Swimming,
        value: 0,
    },
    Speed {
        movement: Movement::Climbing,
        value: 0,
    },
];
