use dregg::app::App;

fn main() -> iced::Result {
    iced::application(App::title(), App::update, App::view).run()
}
