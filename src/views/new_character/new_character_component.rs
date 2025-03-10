#![allow(dead_code)]

use iced::widget::pane_grid;

use super::RaceComponent;

/// Represents the menu and info panes of this page.
#[derive(Debug)]
enum Pane {
    Menu,
    Info,
}

/// The `New Character` page.
#[derive(Debug)]
pub struct NewCharacterPage {
    /// Represents the two panes (menu pane and info pane).
    panes: pane_grid::State<Pane>,

    /// The race state.
    race: RaceComponent,
}
