use gui::app::SqlGui;
use iced::{Sandbox, Settings};

mod gui;

const APP_TITLE: &str = "Lore SQL GUI";

fn main() -> iced::Result {
    SqlGui::run(Settings::default())
}
