use iced::{
    widget::{column, container},
    Element,
};

use crate::{app::Message, views::Component};

#[derive(Default, Debug)]
pub enum Page {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

impl Component for Page {
    type Message = Message;
    type Context = ();
    type Command = ();

    fn view(&self, _ctx: Self::Context) -> Element<Self::Message> {
        container(column![]).into()
    }

    fn update(&mut self, _message: Self::Message) -> Self::Command {}
}
