
use cosmic::{widget, Element, Command};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    TempMessage,
}

pub struct Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self {}
    }
}

impl Settings {
    pub fn view<'a>(&self) -> Element<'a, SettingsMessage> {
        widget::text("settings page").into()
    }
    
    pub fn update(&mut self, message: SettingsMessage) -> Command<crate::app::Message> {
        match message {
            SettingsMessage::TempMessage => {
                println!("temp message");
            },
        }
        Command::none()
    }
    
}