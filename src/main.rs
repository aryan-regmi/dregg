use dregg::components::{
    new_character::{self, NewCharacter},
    race::Race,
};

fn main() -> iced::Result {
    iced::application("Dregg", App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    NewCharacterView(new_character::Message),

    LoadCharacterView,
}

#[derive(Default)]
enum View {
    #[default]
    Main,
    NewCharacter(new_character::NewCharacter),
    LoadCharacter,
}

#[derive(Default)]
struct App {
    /// The current view.
    view: View,

    /// The selected race.
    selected_race: Option<Race>,
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Change to `NewCharacter`
            Message::NewCharacterView(msg) => {
                let mut component = NewCharacter::new(self.selected_race.clone());
                match component.update(msg) {
                    new_character::Action::None => {}

                    // Return to main menu
                    new_character::Action::MainMenu => {
                        self.view = View::Main;
                        return iced::Task::none();
                    }

                    // Update the selected race.
                    new_character::Action::UpdateSelectedRace(race) => {
                        self.selected_race = Some(race)
                    }
                }
                self.view = View::NewCharacter(component);
            }

            // Change to `LoadCharacter`
            Message::LoadCharacterView => {
                self.view = View::LoadCharacter;
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        use iced::widget;
        match &self.view {
            // The main page view
            View::Main => widget::column![
                widget::button("New Character")
                    .on_press(Message::NewCharacterView(new_character::Message::RaceView)),
                widget::button("Load Character").on_press(Message::LoadCharacterView),
            ]
            .into(),

            // The `New Character` view
            View::NewCharacter(component) => component.view().map(Message::NewCharacterView),

            // TODO: Move view to `LoadCharacter` component
            //
            // The `Load Character` view
            View::LoadCharacter => widget::column![widget::button("Main Menu")
                .on_press(Message::NewCharacterView(new_character::Message::MainMenu))]
            .into(),
        }
    }
}
