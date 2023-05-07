use super::Dialog;
use crate::gui::app::message_handling::GuiMes;
use iced::{
    widget::{Button, Column, Text},
    Element, Renderer,
};
use iced_lazy::Component;
use loretex::errors::LoreTexError;

#[derive(Debug, Clone)]
pub(crate) struct ErrorDialog {
    error: LoreTexError,
}

impl Dialog for ErrorDialog {}

impl Component<GuiMes, Renderer> for ErrorDialog {
    type State = ();

    type Event = ErrorDialogMes;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<GuiMes> {
        Some(GuiMes::DialogClosed)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let text = Text::new(self.error.to_string());
        let button = Button::new(Text::new("Ok")).on_press(ErrorDialogMes::Close);
        Column::new().push(text).push(button).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
