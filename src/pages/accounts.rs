use cosmic::{
    iced::{self, Length},
    widget::{self, column, settings},
    Command, Element,
};

use crate::{fl, STORE};

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    TempMessage,
}

pub struct Accounts {
    accounts: Vec<String>,
}

impl Default for Accounts {
    // Initialize default
    fn default() -> Self {
        Self {
            accounts: vec!["Conto corrente".to_string(), "Libretto".to_string()],
        }
    }
}

impl Accounts {
    pub fn view<'a>(&self) -> Element<'a, AccountsMessage> {
        let mut col = column::<AccountsMessage>().push(widget::text::title1(fl!("page_accounts")));

        col = col.push(
            widget::container(
                cosmic::widget::button::text(fl!("add_account"))
                    .on_press(AccountsMessage::TempMessage),
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        );

        for account in &self.accounts {
            col = col
                .push(settings::section().title(account.to_string()).add(
                    widget::row().push(widget::text::text(format!("{}: {} {}", "Balance", 10, "$"))),
                ))
                .push(widget::vertical_space(Length::from(20)))
        }

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: AccountsMessage) -> Command<crate::app::Message> {
        match message {
            AccountsMessage::TempMessage => {
                log::info!("temp logs");
                let mut store = STORE.lock().unwrap();
                store.create_account();
                store.get_accounts();
            }
        }
        Command::none()
    }
}
