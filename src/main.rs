use dregg::app::App;

fn main() -> iced::Result {
    // TODO: Use `iced::application` instead?
    iced::run(App::title, App::update, App::view)
}
