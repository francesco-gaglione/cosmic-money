use chrono::{NaiveDateTime, Utc};
use cosmic::{
    iced::{self, alignment::Vertical, Length, Padding},
    widget::{self, column, settings, Space},
    Element, Task,
};

use crate::{
    app::AppMessage,
    config::Config,
    fl,
    models::{Account, NewAccount, NewAccountTransfer, UpdateAccount},
    widget::date_picker::date_picker,
    STORE,
};

use super::transactions::TransactionMessage;

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    Update,
    AddAccountView,
    TransferMoneyView,
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
    TransferFromAccountChanged(usize),
    TransferToAccountChanged(usize),
    TransferAmountChanged(String),
    TransferDescriptionChanged(String),
    TransferSubmitted,
    TransferCancel,
    TransferDateChanged(i64),
}

pub struct Accounts {
    currency_symbol: String,
    accounts: Vec<Account>,
    add_account_view_visible: bool,
    money_transfer_view_visible: bool,
    form_new_account_name_value: String,
    form_new_account_initial_value: String,
    new_account_initial_value: f32,
    new_account_description: String,
    edit_account_name: String,
    edit_account_balance: String,
    edit_account_description: String,
    editing_account: Option<i32>,
    transfer_from_account: Option<usize>,
    transfer_to_account: Option<usize>,
    transfer_form_amount: String,
    transfer_description: String,
    transfer_amount: f32,
    transfer_date: i64,
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
            money_transfer_view_visible: false,
            form_new_account_name_value: fl!("bank-account"),
            form_new_account_initial_value: String::default(),
            new_account_description: String::default(),
            new_account_initial_value: 0.,
            editing_account: None,
            edit_account_name: String::default(),
            edit_account_balance: String::default(),
            edit_account_description: String::default(),
            transfer_from_account: Some(0),
            transfer_to_account: Some(1),
            transfer_amount: 0.,
            transfer_form_amount: String::default(),
            transfer_date: Utc::now().timestamp(),
            transfer_description: String::default(),
        }
    }
}

impl Accounts {
    pub fn view<'a>(&'a self) -> Element<'a, AccountsMessage> {
        let mut col = column::<AccountsMessage>().push(
            widget::row()
                .align_y(Vertical::Center)
                .push(widget::text::title1(fl!("page_accounts")))
                .push(widget::Space::with_width(Length::Fill))
                .push(widget::text::title4(fl!(
                    "total-balance",
                    balance = format!("{:.2}", self.calc_total_balance()),
                    currency = self.currency_symbol.clone()
                ))),
        );

        col = col.push(
            widget::container(
                widget::column()
                    .push(
                        cosmic::widget::button::text(fl!("add-account"))
                            .on_press(AccountsMessage::AddAccountView),
                    )
                    .push_maybe(if self.accounts.len() > 1 {
                        Some(
                            cosmic::widget::button::text(fl!("transfer"))
                                .on_press(AccountsMessage::TransferMoneyView),
                        )
                    } else {
                        None
                    }),
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        );

        if self.add_account_view_visible {
            col = col.push(self.add_account_view())
        }

        if self.money_transfer_view_visible {
            col = col.push(self.transfer_money_view())
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
                                    format!("{:.2}", self.read_account_balance(account.id)),
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
                    .push(Space::with_height(5))
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

    fn transfer_money_view<'a>(&'a self) -> Element<'a, AccountsMessage> {
        let mut element = widget::column();

        element = element.push(Space::with_height(10));

        element = element.push(
            widget::container(
                widget::column()
                    .push(widget::text::title3(fl!("transfer")))
                    .push(Space::with_height(5))
                    .push(
                        widget::row()
                            .push(widget::text::text(fl!("from")))
                            .push(Space::with_width(5))
                            .push(widget::dropdown(
                                &self.accounts,
                                self.transfer_from_account,
                                AccountsMessage::TransferFromAccountChanged,
                            ))
                            .push(Space::with_width(30))
                            .push(widget::text::text(fl!("to")))
                            .push(Space::with_width(5))
                            .push(widget::dropdown(
                                &self.accounts,
                                self.transfer_to_account,
                                AccountsMessage::TransferToAccountChanged,
                            ))
                            .align_y(Vertical::Center),
                    )
                    .push(Space::with_height(5))
                    .push(date_picker(self.transfer_date, |date| {
                        AccountsMessage::TransferDateChanged(date)
                    }))
                    .push(Space::with_height(5))
                    .push(widget::text::text(fl!("amount")))
                    .push(
                        widget::text_input(fl!("amount"), &self.transfer_form_amount)
                            .width(Length::Fill)
                            .on_input(AccountsMessage::TransferAmountChanged),
                    )
                    .push(Space::with_height(5))
                    .push(widget::text::text(fl!("description")))
                    .push(
                        widget::text_input(fl!("description"), &self.transfer_description)
                            .width(Length::Fill)
                            .on_input(AccountsMessage::TransferDescriptionChanged),
                    )
                    .push(Space::with_height(10))
                    .push(
                        widget::row()
                            .push(
                                widget::button::suggested(fl!("confirm"))
                                    .on_press(AccountsMessage::TransferSubmitted),
                            )
                            .push(Space::with_width(10))
                            .push(
                                widget::button::destructive(fl!("cancel"))
                                    .on_press(AccountsMessage::TransferCancel),
                            ),
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

    pub fn update(&mut self, message: AccountsMessage) -> Task<AppMessage> {
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
            AccountsMessage::TransferMoneyView => {
                self.money_transfer_view_visible = true;
            }
            AccountsMessage::NewBankAccountNameChanged(value) => {
                self.form_new_account_name_value = value;
            }
            AccountsMessage::CancelNewBankAccount => self.add_account_view_visible = false,
            AccountsMessage::NewBankAccountInitialValueChanged(value) => {
                log::info!("value: {:?}", value);
                if value == "" {
                    self.form_new_account_initial_value = String::default();
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
                    name: self.form_new_account_name_value.clone(),
                    initial_balance: self.new_account_initial_value,
                    account_description: self.new_account_description.clone(),
                };
                let mut store = STORE.lock().unwrap();
                let _ = store.create_account(&new_account);
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Accounts(AccountsMessage::Update)
                }));
                self.add_account_view_visible = false;
            }
            AccountsMessage::EditAccount(id) => {
                self.editing_account = Some(id);
                let account = self.accounts.clone().into_iter().find(|a| a.id == id);
                if let Some(account) = account {
                    self.edit_account_name = account.name;
                    self.edit_account_balance = self.read_account_balance(account.id).to_string();
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
                let current_balance = self.read_account_balance(id);
                let account = self.accounts.iter().find(|a| a.id == id);
                match account {
                    Some(account) => {
                        let difference: f32 = new_balance - current_balance;
                        let update_account = UpdateAccount {
                            id,
                            name: self.edit_account_name.clone(),
                            initial_balance: account.initial_balance + difference,
                            account_description: self.edit_account_description.clone(),
                        };
                        let mut store = STORE.lock().unwrap();
                        let _ = store.update_account(&update_account);
                        commands.push(Task::perform(async {}, |_| {
                            AppMessage::Accounts(AccountsMessage::Update)
                        }));
                        commands.push(Task::perform(async {}, |_| {
                            AppMessage::Transactions(TransactionMessage::UpdatePage)
                        }));
                        commands.push(Task::perform(async {}, |_| {
                            AppMessage::Accounts(AccountsMessage::CloseEditAccount)
                        }));
                    }
                    None => {
                        log::error!("Initial balance not found");
                    }
                }
            }
            AccountsMessage::TransferFromAccountChanged(selected) => {
                self.transfer_from_account = Some(selected)
            }
            AccountsMessage::TransferToAccountChanged(selected) => {
                self.transfer_to_account = Some(selected)
            }
            AccountsMessage::TransferAmountChanged(new_amount) => {
                if new_amount.is_empty() {
                    self.transfer_amount = 0.0;
                    self.transfer_form_amount = new_amount;
                } else {
                    match new_amount.parse::<f32>() {
                        Ok(parsed_amount) => {
                            self.transfer_amount = parsed_amount;
                            self.transfer_form_amount = new_amount;
                        }
                        Err(_) => {
                            eprintln!("Failed to parse the amount: {}", new_amount);
                        }
                    }
                }
            }
            AccountsMessage::TransferSubmitted => {
                log::info!("transfer money");
                let mut store = STORE.lock().unwrap();

                let from_account = self.accounts.get(self.transfer_from_account.unwrap());
                let to_account = self.accounts.get(self.transfer_to_account.unwrap());

                if from_account.is_some() && to_account.is_some() {
                    let new_account_transfer = NewAccountTransfer {
                        from_account: from_account.unwrap().id,
                        to_account: to_account.unwrap().id,
                        amount: self.transfer_amount,
                        transfer_date: NaiveDateTime::from_timestamp(self.transfer_date, 0),
                        description: if self.transfer_description.is_empty() {
                            None
                        } else {
                            Some(self.transfer_description.clone())
                        },
                    };
                    let _ = store.create_account_transfer(&new_account_transfer);
                } //TODO else show error toast

                self.transfer_amount = 0.;
                self.transfer_form_amount = String::default();
                self.transfer_from_account = Some(0);
                self.transfer_to_account = Some(1);
                self.money_transfer_view_visible = false;

                commands.push(Task::perform(async {}, |_| AppMessage::UpdateAllPages));
            }
            AccountsMessage::TransferCancel => {
                self.transfer_amount = 0.;
                self.transfer_form_amount = String::default();
                self.transfer_from_account = Some(0);
                self.transfer_to_account = Some(1);
                self.money_transfer_view_visible = false;
            }
            AccountsMessage::TransferDateChanged(date) => {
                self.transfer_date = date;
            }
            AccountsMessage::TransferDescriptionChanged(description) => {
                self.transfer_description = description;
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

    fn calc_total_balance(&self) -> f32 {
        self.accounts
            .iter()
            .map(|a| self.read_account_balance(a.id))
            .sum()
    }
}
