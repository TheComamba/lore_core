use super::app::message_handling::GuiMes;
use iced::{
    widget::{Container, Scrollable, Text},
    Element, Renderer,
};
use iced_aw::{style::CardStyles, Card};
use iced_lazy::{component, Component};

mod error;
mod new_entity;

pub(crate) trait Dialog: Sized + Component<GuiMes, Renderer> {
    fn card_style(self) -> CardStyles {
        CardStyles::Primary
    }

    fn header(self) -> String;

    fn body<'a>(self) -> Element<'a, GuiMes> {
        component(self).into()
    }

    fn to_element<'a>(self) -> Element<'a, GuiMes> {
        let header: Text<'a, Renderer> = Text::new(self.header());
        let body = self.body();
        let card =
            Card::new::<Element<'a, GuiMes>, Element<'a, GuiMes>>(header.into(), body.into())
                .style(self.card_style())
                .on_close(GuiMes::DialogClosed);
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}
