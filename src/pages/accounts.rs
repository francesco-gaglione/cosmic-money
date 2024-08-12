use cosmic::{widget, Element, Command};

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    TempMessage,
}

pub struct Accounts {}

impl Default for Accounts {
    fn default() -> Self {
        Self {}
    }
}

impl Accounts {
    pub fn view<'a>(&self) -> Element<'a, AccountsMessage> {
        widget::text("account page").into()
    }
    
    pub fn update(&mut self, message: AccountsMessage) -> Command<crate::app::Message> {
        match message {
            AccountsMessage::TempMessage => {
                println!("temp message");
            },
        }
        Command::none()
    }
    
}
