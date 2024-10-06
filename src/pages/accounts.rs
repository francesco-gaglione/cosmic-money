use cosmic::{
    iced::{self, Length, Padding},
    widget::{self, column, settings},
    Command, Element,
};

use crate::{
    app, fl,
    models::{Account, NewAccount},
    STORE,
};

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    Update,
    AddAccountView,
    CancelNewBankAccount,
    SubmitNewBankAccount,
    NewBankAccountNameChanged(String),
    NewBankAccountInitialValueChanged(String),
}

pub struct Accounts {
    accounts: Vec<Account>,
    add_account_view_visible: bool,
    form_new_account_name_value: String,
    form_new_account_initial_value: String,
    new_account_initial_value: f32,
}

impl Default for Accounts {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let accounts = store.get_accounts();
        Self {
            accounts: if let Ok(accounts) = accounts {
                accounts
            } else {
                Vec::new()
            },
            add_account_view_visible: false,
            form_new_account_name_value: fl!("bank-account"),
            form_new_account_initial_value: "".to_string(),
            new_account_initial_value: 0.,
        }
    }
}

impl Accounts {
    pub fn view<'a>(&'a self) -> Element<'a, AccountsMessage> {
        let mut col = column::<AccountsMessage>().push(widget::text::title1(fl!("page_accounts")));

        col = col.push(
            widget::container(
                cosmic::widget::button::text(fl!("add-account"))
                    .on_press(AccountsMessage::AddAccountView),
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        );

        if self.add_account_view_visible {
            col = col.push(self.add_account_view())
        }

        if self.accounts.len() > 0 {
            for account in &self.accounts {
                col = col
                    .push(
                        settings::section().title(account.name.to_string()).add(
                            widget::row()
                                .push(widget::text::text(format!("{}: {} {}", "Balance", 10, "$"))), //TODO a partire dall'initial value vai a sommare tutte le transazioni che riguardano questo conto e calcola quindi il bilancio
                        ),
                    )
                    .push(widget::vertical_space(Length::from(20)))
            }
        } else {
            col = col.push(widget::text::text(fl!("no-elements")));
        }

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
            .into()
    }

    fn add_account_view<'a>(&'a self) -> Element<'a, AccountsMessage> {
        let mut element = widget::column();

        element = element.push(widget::vertical_space(Length::from(10)));

        element = element.push(
            widget::container(
                widget::column()
                    .push(widget::text::title3(fl!("new-account")))
                    .push(
                        widget::row()
                            .push(
                                widget::column()
                                    .push(widget::text::text(fl!("account-name")))
                                    .push(
                                        cosmic::widget::text_input(
                                            fl!("bank-account"),
                                            &self.form_new_account_name_value,
                                        )
                                        .on_input(AccountsMessage::NewBankAccountNameChanged),
                                    )
                                    .width(Length::Fill),
                            )
                            .push(widget::horizontal_space(Length::from(10)))
                            .push(
                                widget::column()
                                    .push(widget::text::text(fl!("initial-value")))
                                    .push(
                                        cosmic::widget::text_input(
                                            "0",
                                            &self.form_new_account_initial_value,
                                        )
                                        .on_input(
                                            AccountsMessage::NewBankAccountInitialValueChanged,
                                        ),
                                    )
                                    .width(Length::Fill),
                            ),
                    )
                    .push(widget::vertical_space(Length::from(10)))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("cancel"))
                                    .on_press(AccountsMessage::CancelNewBankAccount)
                                    .style(widget::button::Style::Destructive),
                            )
                            .push(widget::horizontal_space(Length::from(10)))
                            .push(
                                widget::button::text(fl!("add"))
                                    .on_press(AccountsMessage::SubmitNewBankAccount)
                                    .style(widget::button::Style::Suggested),
                            )
                            .width(Length::Fill)
                            .align_items(iced::Alignment::End),
                    )
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .padding(Padding::new(10.))
            .style(cosmic::theme::Container::Card),
        );

        element = element.push(widget::vertical_space(Length::from(10)));

        element.into()
    }

    pub fn update(&mut self, message: AccountsMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            AccountsMessage::Update => {
                log::info!("updating accounts");
                let mut store = STORE.lock().unwrap();
                let accounts = store.get_accounts();
                if let Ok(accounts) = accounts {
                    self.accounts = accounts;
                }
            }
            AccountsMessage::AddAccountView => {
                self.add_account_view_visible = true;
            }
            AccountsMessage::NewBankAccountNameChanged(value) => {
                self.form_new_account_name_value = value;
            }
            AccountsMessage::CancelNewBankAccount => self.add_account_view_visible = false,
            AccountsMessage::NewBankAccountInitialValueChanged(value) => {
                log::info!("value: {:?}", value);
                if value == "" {
                    self.form_new_account_initial_value = "".to_string();
                    self.new_account_initial_value = 0.;
                }
                match value.parse::<f32>() {
                    Ok(float_value) => {
                        self.form_new_account_initial_value = value;
                        self.new_account_initial_value = float_value;
                    }
                    Err(_) => {
                        log::error!("error parsing the initial value")
                    }
                }
            }
            AccountsMessage::SubmitNewBankAccount => {
                let new_account = NewAccount {
                    name: self.form_new_account_name_value.as_str(),
                    account_type: "",
                    initial_balance: self.new_account_initial_value,
                };
                let mut store = STORE.lock().unwrap();
                store.create_account(&new_account);
                commands.push(Command::perform(async {}, |_| {
                    app::Message::Accounts(AccountsMessage::Update)
                }));
                self.add_account_view_visible = false;
            }
        }
        Command::batch(commands)
    }
}
