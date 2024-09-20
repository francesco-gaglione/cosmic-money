use cosmic::{
    iced,
    widget::{self},
    Command, Element,
};

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
        let button =
            cosmic::widget::button::text("test button").on_press(AccountsMessage::TempMessage);

        widget::container(button)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_x()
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: AccountsMessage) -> Command<crate::app::Message> {
        match message {
            AccountsMessage::TempMessage => {
                println!("temp message");
            }
        }
        Command::none()
    }
}
