use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let title = "    Dregg - DnD Character Creator";
        Label::new(cx, title)
            .width(Percentage(18.0))
            // .width(Pixels((title.len() * 7) as f32))
            .border_width(Pixels(2.0))
            .border_color(Color::blue())
            .background_color(Color::red());
    })
    .title("Dregg")
    .inner_size((720, 480))
    .run()
}
