use crate::common::{Trait, TraitEffect, Vision};

/// Represents the `Darkvision` trait.
pub fn darkvision(distance: u16) -> Trait {
    Trait {
        name: "Darkvision".into(),
        summary: "You have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray".into(),
        effects: vec![TraitEffect::Vision(Vision::Darkvision(distance))],
        required_level: None,
        tags: vec!["vision".into()],
    }
}
