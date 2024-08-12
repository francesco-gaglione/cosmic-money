use cosmic::{widget, Command, Element};

#[derive(Debug, Clone)]
pub enum StatisticMessage {
    TempMessage,
}

pub struct Statistics {}

impl Default for Statistics {
    fn default() -> Self {
        Self {}
    }
}

impl Statistics {
    pub fn view<'a>(&self) -> Element<'a, StatisticMessage> {
        widget::text("stat page").into()
    }

    pub fn update(&mut self, message: StatisticMessage) -> Command<crate::app::Message> {
        match message {
            StatisticMessage::TempMessage => {
                println!("temp message");
            }
        }
        Command::none()
    }
}
