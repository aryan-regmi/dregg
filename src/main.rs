use dregg::components::{
    custom_race_creator::{self, CustomRaceCreator},
    new_character::{self, NewCharacter},
    race::Race,
};
use iced::widget;

fn main() -> iced::Result {
    iced::application("Dregg", App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    NewCharacterView(new_character::Message),

    LoadCharacterView,

    RaceCreator(custom_race_creator::Message),
}

#[derive(Default)]
enum View {
    #[default]
    Main,
    NewCharacter(new_character::NewCharacter),
    LoadCharacter,
    CustomRaceCreator(custom_race_creator::CustomRaceCreator),
}

#[derive(Default)]
struct App {
    /// The current view.
    view: View,

    /// The selected race.
    selected_race: Option<Race>,

    /// Summary for the race creator.
    race_creator_summary: String,
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Create and update `NewCharacterView`
            Message::NewCharacterView(msg) => {
                let mut component = NewCharacter::new(self.selected_race.clone());
                match component.update(msg) {
                    new_character::Action::None => {}

                    // Return to main menu
                    new_character::Action::MainMenu => {
                        self.view = View::Main;
                        return iced::Task::none();
                    }

                    // Update the selected race
                    new_character::Action::UpdateSelectedRace(race) => {
                        self.selected_race = Some(race)
                    }

                    // Open the custom race creator
                    new_character::Action::OpenCustomRaceCreator => {
                        // TODO: Call update func of `race_creator`
                        let race_creator = CustomRaceCreator::new(&self.race_creator_summary);
                        self.view = View::CustomRaceCreator(race_creator);
                        return iced::Task::none();
                    }
                }
                self.view = View::NewCharacter(component);
            }

            // Create and update `LoadCharacterView`
            Message::LoadCharacterView => {
                self.view = View::LoadCharacter;
            }

            // Create and update `CustomRaceCreator`
            Message::RaceCreator(message) => {
                if let View::CustomRaceCreator(component) = &mut self.view {
                    match component.update(message) {
                        custom_race_creator::Action::None => {}

                        // Update the summary text
                        custom_race_creator::Action::UpdateSummaryText(txt) => {
                            self.race_creator_summary = txt
                        }

                        // Create the custom race and return to the `NewCharacterView`
                        custom_race_creator::Action::CreateAndReturn => {
                            let new_character = NewCharacter::new(self.selected_race.clone());
                            self.view = View::NewCharacter(new_character);
                        }
                    }
                }
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        use iced::widget;
        match &self.view {
            View::Main => widget::column![
                widget::button("New Character")
                    .on_press(Message::NewCharacterView(new_character::Message::RaceView)),
                widget::button("Load Character").on_press(Message::LoadCharacterView),
            ]
            .into(),

            View::NewCharacter(component) => component.view().map(Message::NewCharacterView),

            View::LoadCharacter => widget::column![widget::button("Main Menu")
                .on_press(Message::NewCharacterView(new_character::Message::MainMenu))]
            .into(),

            View::CustomRaceCreator(component) => {
                widget::column![component.view().map(Message::RaceCreator)]
                    .padding(10)
                    .into()
            }
        }
    }
}
