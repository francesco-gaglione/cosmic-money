use cosmic::{
    iced::{self, Length, Padding},
    widget::{self, column, settings, Space},
    Element, Task,
};

use crate::{
    app,
    config::Config,
    fl,
    models::{Account, NewAccount, UpdateAccount},
    STORE,
};

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    Update,
    AddAccountView,
    CancelNewBankAccount,
    SubmitNewBankAccount,
    EditAccount(i32),
    CloseEditAccount,
    NewAccountDescriptionChanged(String),
    EditAccountName(String),
    EditAccountBalance(String),
    EditAccountDescription(String),
    EditAccountSubmit,
    NewBankAccountNameChanged(String),
    NewBankAccountInitialValueChanged(String),
}

pub struct Accounts {
    currency_symbol: String,
    accounts: Vec<Account>,
    add_account_view_visible: bool,
    form_new_account_name_value: String,
    form_new_account_initial_value: String,
    new_account_initial_value: f32,
    new_account_description: String,
    edit_account_name: String,
    edit_account_balance: String,
    edit_account_description: String,
    editing_account: Option<i32>,
}

impl Default for Accounts {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let config = Config::load();

        let accounts = store.get_accounts();
        let currency_symbol = store.get_currency_symbol_by_id(config.1.currency_id);

        Self {
            currency_symbol: currency_symbol.unwrap_or_else(|_| "USD".to_string()),
            accounts: if let Ok(accounts) = accounts {
                accounts
            } else {
                Vec::new()
            },
            add_account_view_visible: false,
            form_new_account_name_value: fl!("bank-account"),
            form_new_account_initial_value: "".to_string(),
            new_account_description: "".to_string(),
            new_account_initial_value: 0.,
            editing_account: None,
            edit_account_name: "".to_string(),
            edit_account_balance: "".to_string(),
            edit_account_description: "".to_string(),
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
                let edit_button = widget::button::icon(widget::icon::from_name("edit-symbolic"))
                    .on_press(AccountsMessage::EditAccount(account.id));
                let mut main_col = widget::column().push(
                    widget::row()
                        .push(
                            widget::column()
                                .push(widget::text::text(format!(
                                    "{}: {} {}",
                                    "Balance",
                                    self.read_account_balance(account.id),
                                    self.currency_symbol
                                )))
                                .width(Length::Fill),
                        )
                        .push(match self.editing_account {
                            Some(id) => {
                                if id == account.id {
                                    widget::button::icon(widget::icon::from_name(
                                        "window-close-symbolic",
                                    ))
                                    .on_press(AccountsMessage::CloseEditAccount)
                                } else {
                                    edit_button
                                }
                            }
                            None => edit_button,
                        })
                        .width(Length::Fill),
                );
                if let Some(account_id) = self.editing_account {
                    if account_id == account.id {
                        main_col = main_col.push(widget::divider::horizontal::default());
                        main_col = main_col.push(Space::with_height(10));
                        main_col = main_col.push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::title4(fl!("account-name")))
                                        .push(
                                            widget::text_input(
                                                fl!("account-name"),
                                                &self.edit_account_name,
                                            )
                                            .on_input(AccountsMessage::EditAccountName),
                                        )
                                        .width(Length::Fill),
                                )
                                .push(Space::with_width(10))
                                .push(
                                    widget::column()
                                        .push(widget::text::title4(fl!("balance")))
                                        .push(
                                            widget::text_input(
                                                fl!("balance"),
                                                &self.edit_account_balance,
                                            )
                                            .on_input(AccountsMessage::EditAccountBalance),
                                        )
                                        .width(Length::Fill),
                                ),
                        );
                        main_col = main_col.push(Space::with_height(10));
                        main_col = main_col.push(
                            widget::column()
                                .push(widget::text::text(fl!("description")))
                                .push(
                                    widget::text_input(
                                        fl!("description"),
                                        &self.edit_account_description,
                                    )
                                    .on_input(AccountsMessage::EditAccountDescription),
                                ),
                        );
                        main_col = main_col.push(Space::with_height(10));
                        main_col = main_col.push(
                            widget::row()
                                .push(
                                    widget::button::text(fl!("save"))
                                        .on_press(AccountsMessage::EditAccountSubmit)
                                        .class(widget::button::ButtonClass::Suggested),
                                )
                                .push(Space::with_width(10))
                                .push(
                                    widget::button::text(fl!("cancel"))
                                        .on_press(AccountsMessage::CloseEditAccount)
                                        .class(widget::button::ButtonClass::Destructive),
                                ),
                        )
                    }
                }
                col = col
                    .push(
                        settings::section()
                            .title(account.name.to_string())
                            .add(main_col),
                    )
                    .push(Space::with_height(20));
            }
        } else {
            col = col.push(widget::text::text(fl!("no-elements")));
        }

        widget::scrollable(
            widget::container(col)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink),
        )
        .into()
    }

    fn add_account_view<'a>(&'a self) -> Element<'a, AccountsMessage> {
        let mut element = widget::column();

        element = element.push(Space::with_height(10));

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
                            .push(Space::with_width(10))
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
                    .push(Space::with_height(10))
                    .push(
                        widget::column()
                            .width(Length::Fill)
                            .push(widget::text::text(fl!("description")))
                            .push(
                                widget::text_input(
                                    fl!("description"),
                                    &self.new_account_description,
                                )
                                .width(Length::Fill)
                                .on_input(AccountsMessage::NewAccountDescriptionChanged),
                            ),
                    )
                    .push(Space::with_height(10))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("cancel"))
                                    .on_press(AccountsMessage::CancelNewBankAccount)
                                    .class(widget::button::ButtonClass::Destructive),
                            )
                            .push(Space::with_width(10))
                            .push(
                                widget::button::text(fl!("add"))
                                    .on_press(AccountsMessage::SubmitNewBankAccount)
                                    .class(widget::button::ButtonClass::Suggested),
                            )
                            .width(Length::Fill)
                            .align_y(iced::Alignment::End),
                    )
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .padding(Padding::new(10.))
            .class(cosmic::theme::Container::Card),
        );

        element = element.push(Space::with_height(10));

        element.into()
    }

    pub fn update(&mut self, message: AccountsMessage) -> Task<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            AccountsMessage::Update => {
                log::info!("updating accounts");
                let mut store = STORE.lock().unwrap();
                let config = Config::load();
                let accounts = store.get_accounts();
                let currency_symbol = store.get_currency_symbol_by_id(config.1.currency_id);
                if let Ok(accounts) = accounts {
                    self.accounts = accounts;
                    self.currency_symbol = currency_symbol.unwrap_or_else(|_| "USD".to_string());
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
                    account_description: self.new_account_description.clone(),
                };
                let mut store = STORE.lock().unwrap();
                store.create_account(&new_account);
                commands.push(Task::perform(async {}, |_| {
                    app::Message::Accounts(AccountsMessage::Update)
                }));
                self.add_account_view_visible = false;
            }
            AccountsMessage::EditAccount(id) => {
                self.editing_account = Some(id);
                let account = self.accounts.clone().into_iter().find(|a| a.id == id);
                if let Some(account) = account {
                    self.edit_account_name = account.name;
                    self.edit_account_balance = account.initial_balance.to_string();
                    self.edit_account_description = account.account_description;
                }
            }
            AccountsMessage::CloseEditAccount => self.editing_account = None,
            AccountsMessage::NewAccountDescriptionChanged(value) => {
                self.new_account_description = value;
            }
            AccountsMessage::EditAccountName(new_name) => {
                self.edit_account_name = new_name;
            }
            AccountsMessage::EditAccountBalance(new_balance) => {
                if new_balance.parse::<f32>().is_ok() || new_balance == "" {
                    self.edit_account_balance = new_balance;
                }
            }
            AccountsMessage::EditAccountDescription(new_description) => {
                self.edit_account_description = new_description;
            }
            AccountsMessage::EditAccountSubmit => {
                let id = self.editing_account.unwrap();
                let new_balance = self.edit_account_balance.parse::<f32>().unwrap();
                let initial_balance = self.read_account_balance(id);
                let difference: f32 = new_balance - initial_balance;
                let update_account = UpdateAccount {
                    id,
                    name: self.edit_account_name.as_str(),
                    initial_balance: initial_balance + difference,
                    account_description: self.edit_account_description.clone(),
                };
                let mut store = STORE.lock().unwrap();
                let _ = store.update_account(&update_account);
                commands.push(Task::perform(async {}, |_| {
                    app::Message::Accounts(AccountsMessage::Update)
                }));
                commands.push(Task::perform(async {}, |_| {
                    app::Message::Accounts(AccountsMessage::CloseEditAccount)
                }));
            }
        }
        Task::batch(commands)
    }

    fn read_account_balance(&self, account_id: i32) -> f32 {
        let mut store = STORE.lock().unwrap();
        if let Ok(balance) = store.get_account_balance(account_id) {
            balance
        } else {
            0.
        }
    }
}
