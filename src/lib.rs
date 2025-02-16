use frontend::new_char_page::NewCharPagePane;
use iced::widget::pane_grid;

pub mod frontend;

pub struct State {
    pub screen: Screen,
    panes: pane_grid::State<NewCharPagePane>,
}

impl Default for State {
    fn default() -> Self {
        let (mut panes, pane) = pane_grid::State::new(NewCharPagePane::Menu);
        panes.split(pane_grid::Axis::Vertical, pane, NewCharPagePane::Main);
        Self {
            screen: Default::default(),
            panes,
        }
    }
}

#[derive(Default)]
pub enum Screen {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

#[derive(Debug, Clone)]
pub enum Message {
    Main,
    LoadCharacter,
    NewCharacter,
}
