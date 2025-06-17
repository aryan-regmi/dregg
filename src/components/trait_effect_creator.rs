use iced::widget;

use crate::utils;

#[derive(Debug, Clone)]
pub enum Message {
    /// The vision type has been selected.
    VisionRadioSelected(utils::Vision),

    /// Back button pressed.
    BackButtonPressed,

    /// Create button pressed.
    CreateButtonPressed,
}

pub enum Action {
    /// No action required.
    None,

    /// Cancel adding a new trait effect.
    Cancel,

    /// Create and return the trait effect.
    Create(utils::TraitEffect),
}

#[derive(Debug)]
pub struct TraitEffectCreator {
    /// The trait effect to display.
    effect: utils::TraitEffect,

    /// Currently selected vision radio button.
    selected_vision: Option<utils::Vision>,
}

impl TraitEffectCreator {
    /// Creates a new `TraitEffectCreator`.
    pub fn new(effect: utils::TraitEffect) -> Self {
        Self {
            effect,
            selected_vision: None,
        }
    }

    /// Returns a list of all vision types.
    fn all_visions() -> Vec<utils::Vision> {
        vec![
            utils::Vision::Normal(0),
            utils::Vision::Darkvision(0),
            utils::Vision::Truesight(0),
            utils::Vision::DevilsSight(0),
        ]
    }
}

impl TraitEffectCreator {
    pub fn view(&self) -> iced::Element<Message> {
        let mut content = widget::column![];

        match &self.effect {
            utils::TraitEffect::Vision(_) => {
                let mut inner = widget::row![];
                for vision in Self::all_visions() {
                    let radio = widget::radio(
                        vision.to_string(),
                        vision,
                        self.selected_vision,
                        Message::VisionRadioSelected,
                    );
                    inner = inner.push(radio);
                }

                if let Some(selected_vision) = &self.selected_vision {
                    let input = widget::text_input("", &selected_vision.get_value().to_string());
                    inner = inner.push(input);
                }

                content = content.push(inner);
            }

            utils::TraitEffect::SavingThrows { advantage, kind } => todo!(),

            utils::TraitEffect::Resistances(resistance) => todo!(),
            utils::TraitEffect::Proficiencies(choice) => todo!(),
            utils::TraitEffect::Spell(spell) => todo!(),
            utils::TraitEffect::Action { kind, effects } => todo!(),
            utils::TraitEffect::HpIncrease(hp_increase) => todo!(),
            utils::TraitEffect::NoSpeedReduction => todo!(),
        }

        // Add navigation buttons
        {
            let navigation_buttons = widget::container(widget::row![
                widget::button("Back").on_press(Message::BackButtonPressed),
                widget::button("Create").on_press(Message::CreateButtonPressed),
            ]);
            content = content.push(navigation_buttons);
        }

        widget::container(content).into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::VisionRadioSelected(vision) => {
                self.selected_vision = Some(vision);
                self.effect = utils::TraitEffect::Vision(vision);
                Action::None
            }
            Message::BackButtonPressed => Action::Cancel,
            Message::CreateButtonPressed => Action::Create(self.effect.clone()),
        }
    }
}
