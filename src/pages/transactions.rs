use cosmic::{widget, Command, Element};

#[derive(Debug, Clone)]
pub enum TransactionMessage {
    TempMessage,
}

pub struct Transactions {}

impl Default for Transactions {
    fn default() -> Self {
        Self {}
    }
}

impl Transactions {
    pub fn view<'a>(&self) -> Element<'a, TransactionMessage> {
        widget::text("transactions page").into()
    }

    pub fn update(&mut self, message: TransactionMessage) -> Command<crate::app::Message> {
        match message {
            TransactionMessage::TempMessage => {
                println!("temp message");
            }
        }
        Command::none()
    }
}
