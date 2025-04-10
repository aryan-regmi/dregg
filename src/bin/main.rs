use dregg::app::App;
use dregg::views::Component;

fn main() -> iced::Result {
    // TODO: Use `iced::application` instead?
    iced::run(App::TITLE, App::update, App::view_fixed)
}
