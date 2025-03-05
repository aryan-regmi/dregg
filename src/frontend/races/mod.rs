pub mod dwarf;

pub mod common {
    use crate::frontend::race::RacialTrait;
    
    pub fn darkvision() -> RacialTrait {
        RacialTrait { 
            name: "Darkvision".into(),
            summary: "You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.".into(),
            action_type: None 
        }
    }
}

