use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use cosmic::{
    iced::{Alignment, Length, Padding},
    widget::{
        self, column,
        segmented_button::{self, SingleSelect},
        tab_bar, text_input, Space,
    },
    Element, Task,
};

use crate::{
    app::AppMessage,
    config::Config,
    fl,
    models::{Account, AccountTransfer, Category, MoneyTransaction, NewMoneyTransaction},
    utils::dates::get_month_date_range,
    widget::date_picker::date_picker,
    STORE,
};

pub enum ViewItem {
    Transactions,
    Transfers,
}

#[derive(Debug, Clone)]
pub enum TransactionMessage {
    UpdatePage,
    AddTransaction,
    FormCategoryChanged(usize),
    FormBankAccountChanged(usize),
    FormTransactionTypeChanged(widget::segmented_button::Entity),
    FormNoteChanged(String),
    FormAmountChanged(String),
    FormDateChanged(i64),
    CandellAddTransaction,
    SubmitTransaction,
    PreviousMonth,
    NextMonth,
    ViewChanged(segmented_button::Entity),
}

pub struct Transactions {
    month_names: Vec<String>,
    currency_symbol: String,
    add_transaction_view: bool,
    all_categories: Vec<Category>,
    categories: Vec<Category>,
    accounts: Vec<Account>,
    form_transaction_type: widget::segmented_button::SingleSelectModel,
    form_selectected_category: Option<usize>,
    transactions: Vec<MoneyTransaction>,
    transfers: Vec<AccountTransfer>,
    form_selected_bank_account: Option<usize>,
    form_note: String,
    form_amount: String,
    form_date: i64,
    new_transaction_amount: f32,
    view_month: u32,
    view_year: i32,
    view_selection: widget::segmented_button::SingleSelectModel,
}

impl Default for Transactions {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let config = Config::load();

        let now = Local::now();
        let (start_date, end_date) = get_month_date_range(now.year(), now.month());
        let transactions = store
            .get_money_transactions_date_range(&start_date, &end_date)
            .unwrap_or_else(|_| vec![]);
        let transfers = store
            .get_transfers_date_range(&start_date, &end_date)
            .unwrap_or_else(|_| vec![]);
        log::info!("transfers date rage: {:?}", transfers);
        let currency_symbol = store.get_currency_symbol_by_id(config.1.currency_id);
        let all_categories = store.get_categories().unwrap_or_else(|_| vec![]);
        let categories = all_categories
            .iter()
            .filter(|c| !c.is_income)
            .cloned()
            .collect();
        Self {
            month_names: vec![
                fl!("month-1"),  // January
                fl!("month-2"),  // February
                fl!("month-3"),  // March
                fl!("month-4"),  // April
                fl!("month-5"),  // May
                fl!("month-6"),  // June
                fl!("month-7"),  // July
                fl!("month-8"),  // August
                fl!("month-9"),  // September
                fl!("month-10"), // October
                fl!("month-11"), // November
                fl!("month-12"), // December
            ],
            currency_symbol: currency_symbol.unwrap_or_else(|_| "USD".to_string()),
            add_transaction_view: false,
            all_categories,
            categories,
            accounts: store.get_accounts().unwrap_or_else(|_| vec![]),
            form_transaction_type: widget::segmented_button::Model::builder()
                .insert(|b| b.text(fl!("expense")).data(1u16).activate())
                .insert(|b| b.text(fl!("income")).data(2u16))
                .build(),
            form_note: "".to_string(),
            form_selectected_category: Some(0),
            form_selected_bank_account: Some(0),
            transactions,
            transfers,
            form_amount: "".to_string(),
            form_date: Utc::now().timestamp(),
            new_transaction_amount: 0.,
            view_month: now.month(),
            view_year: now.year(),
            view_selection: widget::segmented_button::Model::<SingleSelect>::builder()
                .insert(|b| {
                    b.text(fl!("transactions"))
                        .data(ViewItem::Transactions)
                        .activate()
                })
                .insert(|b| b.text(fl!("transfers")).data(ViewItem::Transfers))
                .build()
                .into(),
        }
    }
}

impl Transactions {
    pub fn view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let container = widget::container(if self.add_transaction_view {
            self.new_transaction_view()
        } else {
            self.header_view()
        })
        .padding(Padding::new(15.));
        widget::scrollable(container).into()
    }

    pub fn header_view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let mut element: widget::Column<'a, TransactionMessage> = widget::column()
            .push(
                widget::row()
                    .push(
                        widget::column()
                            .push(widget::text::title1(fl!("transactions")))
                            .width(Length::Fill),
                    )
                    .push(
                        widget::column()
                            .push(
                                widget::button::text(fl!("add-transaction"))
                                    .on_press(TransactionMessage::AddTransaction)
                                    .class(widget::button::ButtonClass::Suggested),
                            )
                            .width(Length::Fill)
                            .align_x(Alignment::End),
                    ),
            )
            .width(Length::Fill)
            .align_x(Alignment::Start);

        element = element.push(
            widget::column()
                .push(
                    widget::row()
                        .push(
                            widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                                .on_press(TransactionMessage::PreviousMonth),
                        )
                        .push(Space::with_width(10))
                        .push(
                            widget::container(
                                widget::row()
                                    .push(widget::text::text(
                                        self.month_names[self.view_month as usize - 1].clone(),
                                    ))
                                    .push(Space::with_width(10))
                                    .push(widget::text::text(self.view_year.to_string())),
                            )
                            .padding(Padding::from(7)),
                        )
                        .push(Space::with_width(10))
                        .push(
                            widget::button::icon(widget::icon::from_name("go-next-symbolic"))
                                .on_press(TransactionMessage::NextMonth),
                        ),
                )
                .align_x(Alignment::Center)
                .width(Length::Fill),
        );

        element = element.push(Space::with_height(10));
        element = element.push(
            tab_bar::horizontal(&self.view_selection).on_activate(TransactionMessage::ViewChanged),
        );
        element = element.push(Space::with_height(10));

        element = element.push(match self.view_selection.active_data() {
            Some(ViewItem::Transactions) => self.transactions_view(),
            Some(ViewItem::Transfers) => self.transfers_view(),
            _ => self.transactions_view(),
        });

        element.into()
    }

    pub fn transactions_view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let mut element = widget::column();

        if !self.transactions.is_empty() {
            let mut last_date: NaiveDateTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);

            for t in &self.transactions {
                let mut date_row: Option<Element<'a, TransactionMessage>> = None;
                if t.transaction_date.date().ne(&last_date.date()) {
                    let month = t.transaction_date.month();

                    date_row = Some(
                        widget::row()
                            .push(widget::text::title4(format!(
                                "{} {}",
                                t.transaction_date.day().to_string(),
                                self.month_names[month as usize - 1]
                            )))
                            .into(),
                    );
                    last_date = t.transaction_date.clone();
                }
                element = element.push_maybe(date_row);
                let container = widget::container(
                    widget::column()
                        .push(
                            widget::row()
                                .push(
                                    widget::text::text(format!(
                                        "{}: {}{} {}",
                                        fl!("amount"),
                                        if t.is_expense { "-" } else { "+" },
                                        t.amount,
                                        self.currency_symbol
                                    ))
                                    .width(Length::Fill),
                                )
                                .push(
                                    widget::text::text(format!(
                                        "{}: {}",
                                        fl!("category"),
                                        self.categories
                                            .iter()
                                            .find(|c| c.id == t.transaction_category)
                                            .map(|c| c.name.clone())
                                            .unwrap_or_else(|| fl!("not-found"))
                                    ))
                                    .width(Length::Fill),
                                )
                                .push(
                                    widget::text::text(format!(
                                        "{}: {}",
                                        fl!("date"),
                                        Local
                                            .from_utc_datetime(&t.transaction_date)
                                            .format("%d-%m-%Y %H:%M")
                                            .to_string()
                                    ))
                                    .width(Length::Fill),
                                )
                                .width(Length::Fill),
                        )
                        .push(Space::with_height(5))
                        .push_maybe(if !t.description.is_empty() {
                            Some(widget::row().push(widget::text::text(format!(
                                "{}: {}",
                                fl!("note"),
                                t.description
                            ))))
                        } else {
                            None
                        })
                        .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding(Padding::new(10.))
                .class(cosmic::theme::Container::Card);

                element = element.push(container).push(Space::with_height(10))
            }
        } else {
            element = element.push(widget::text::text(fl!("no-elements")))
        }

        element.into()
    }

    pub fn transfers_view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let mut element = widget::column();

        if !self.transfers.is_empty() {
            let mut last_date: NaiveDateTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);

            for t in &self.transfers {
                let mut date_row: Option<Element<'a, TransactionMessage>> = None;
                if t.transfer_date.date().ne(&last_date.date()) {
                    let month = t.transfer_date.month();

                    date_row = Some(
                        widget::row()
                            .push(widget::text::title4(format!(
                                "{} {}",
                                t.transfer_date.day().to_string(),
                                self.month_names[month as usize - 1]
                            )))
                            .into(),
                    );
                    last_date = t.transfer_date.clone();
                }
                element = element.push_maybe(date_row);
                let container = widget::container(
                    widget::column()
                        .push(
                            widget::row()
                                .width(Length::Fill)
                                .push(
                                    widget::column()
                                        .push(widget::text::text(format!(
                                            "{} {}",
                                            fl!("from"),
                                            self.accounts
                                                .iter()
                                                .find(|a| a.id == t.from_account)
                                                .unwrap()
                                                .name
                                                .clone()
                                        )))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(format!(
                                            "{} {}",
                                            fl!("to"),
                                            self.accounts
                                                .iter()
                                                .find(|a| a.id == t.to_account)
                                                .unwrap()
                                                .name
                                                .clone()
                                        )))
                                        .width(Length::Fill),
                                ),
                        )
                        .push(
                            widget::row()
                                .push(widget::column().width(Length::Fill).push(
                                    widget::text::text(format!(
                                        "{}: {} {}",
                                        fl!("amount"),
                                        t.amount,
                                        self.currency_symbol
                                    )),
                                ))
                                .push(widget::column().width(Length::Fill).push(
                                    widget::text::text(format!(
                                            "{}: {}",
                                            fl!("date"),
                                            Local
                                                .from_utc_datetime(&t.transfer_date)
                                                .format("%d-%m-%Y %H:%M")
                                                .to_string()
                                        )),
                                ))
                                .width(Length::Fill),
                        )
                        .push(Space::with_height(5))
                        .push_maybe(if t.description.is_some() {
                            Some(widget::row().push(widget::text::text(format!(
                                "{}: {}",
                                fl!("note"),
                                t.description.clone().unwrap()
                            ))))
                        } else {
                            None
                        })
                        .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding(Padding::new(10.))
                .class(cosmic::theme::Container::Card);

                element = element.push(container).push(Space::with_height(10))
            }
        } else {
            element = element.push(widget::text::text(fl!("no-elements")))
        }

        element.into()
    }

    pub fn new_transaction_view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let mut element = widget::column().width(Length::Fill);

        element = element.push(widget::text::title1(fl!("add-transaction")));

        element = element.push(Space::with_height(10));

        element = element.push(
            widget::segmented_control::horizontal(&self.form_transaction_type)
                .on_activate(TransactionMessage::FormTransactionTypeChanged),
        );

        element = element.push(Space::with_height(10));

        element = element.push(
            widget::column()
                .push(widget::text::text(fl!("amount")))
                .push(
                    text_input(fl!("amount"), &self.form_amount)
                        .width(Length::Fill)
                        .on_input(TransactionMessage::FormAmountChanged),
                ),
        );

        element = element.push(Space::with_height(10));

        element = element
            .push(widget::text::text(fl!("date")))
            .push(Space::with_height(5));
        element = element.push(date_picker(self.form_date, |date| {
            TransactionMessage::FormDateChanged(date)
        }));

        element = element.push(Space::with_height(10));

        element = element
            .push(
                widget::row()
                    .push(
                        widget::column()
                            .push(widget::text::text(fl!("category")))
                            .push(Space::with_height(Length::from(5)))
                            .push(widget::dropdown(
                                &self.categories,
                                self.form_selectected_category,
                                TransactionMessage::FormCategoryChanged,
                            )),
                    )
                    .push(Space::with_width(Length::from(20)))
                    .push(
                        widget::column()
                            .push(widget::text::text(fl!("bank-account")))
                            .push(widget::dropdown(
                                &self.accounts,
                                self.form_selected_bank_account,
                                TransactionMessage::FormBankAccountChanged,
                            )),
                    ),
            )
            .push(
                column().push(widget::text::text(fl!("note"))).push(
                    text_input(fl!("note"), &self.form_note)
                        .width(Length::Fill)
                        .on_input(TransactionMessage::FormNoteChanged),
                ),
            );

        element = element
            .push(widget::vertical_space().height(Length::from(10)))
            .push(
                widget::row()
                    .push(
                        widget::button::text(fl!("add-transaction"))
                            .on_press(TransactionMessage::SubmitTransaction)
                            .class(widget::button::ButtonClass::Suggested),
                    )
                    .push(widget::horizontal_space().width(Length::from(10)))
                    .push(
                        widget::button::text(fl!("cancel"))
                            .on_press(TransactionMessage::CandellAddTransaction)
                            .class(widget::button::ButtonClass::Destructive),
                    ),
            );

        element.into()
    }

    pub fn update(&mut self, message: TransactionMessage) -> Task<AppMessage> {
        let mut commands = Vec::new();
        match message {
            TransactionMessage::UpdatePage => {
                log::info!("updating page");
                let mut store = STORE.lock().unwrap();
                let config = Config::load();
                let (start_date, end_date) = get_month_date_range(self.view_year, self.view_month);

                let currency_symbol = store.get_currency_symbol_by_id(config.1.currency_id);
                self.transactions = store
                    .get_money_transactions_date_range(&start_date, &end_date)
                    .unwrap_or_else(|_| vec![]);

                self.transfers = store
                    .get_transfers_date_range(&start_date, &end_date)
                    .unwrap_or_else(|_| vec![]);

                let all_categories = store.get_categories().unwrap_or_else(|_| vec![]);
                self.categories = all_categories
                    .iter()
                    .filter(|c| !c.is_income)
                    .cloned()
                    .collect();
                self.form_transaction_type.activate_position(0);

                self.accounts = store.get_accounts().unwrap_or_else(|_| vec![]);
                self.currency_symbol = currency_symbol.unwrap_or_else(|_| "USD".to_string());
            }
            TransactionMessage::AddTransaction => {
                self.add_transaction_view = true;
            }
            TransactionMessage::FormCategoryChanged(selected) => {
                self.form_selectected_category = Some(selected)
            }
            TransactionMessage::FormTransactionTypeChanged(key) => {
                self.form_transaction_type.activate(key);
                let mut is_expense: bool = true;
                if let Some(id) = self
                    .form_transaction_type
                    .data::<u16>(self.form_transaction_type.active())
                {
                    if id == &2 {
                        is_expense = false;
                    }
                }

                self.categories = self
                    .all_categories
                    .iter()
                    .filter(|c| c.is_income == !is_expense)
                    .cloned()
                    .collect();
            }
            TransactionMessage::FormBankAccountChanged(selected) => {
                self.form_selected_bank_account = Some(selected);
            }
            TransactionMessage::FormNoteChanged(note) => {
                self.form_note = note;
            }
            TransactionMessage::FormAmountChanged(new_amount) => {
                if new_amount.is_empty() {
                    self.new_transaction_amount = 0.0;
                    self.form_amount = new_amount;
                } else {
                    match new_amount.parse::<f32>() {
                        Ok(parsed_amount) => {
                            self.new_transaction_amount = parsed_amount;
                            self.form_amount = new_amount;
                        }
                        Err(_) => {
                            eprintln!("Failed to parse the amount: {}", new_amount);
                        }
                    }
                }
            }
            TransactionMessage::SubmitTransaction => {
                let mut is_expense: bool = true;
                if let Some(id) = self
                    .form_transaction_type
                    .data::<u16>(self.form_transaction_type.active())
                {
                    if id == &2 {
                        is_expense = false;
                    }
                }
                let mut store = STORE.lock().unwrap();
                let new_transaction = NewMoneyTransaction {
                    bank_account: self
                        .accounts
                        .get(self.form_selected_bank_account.unwrap())
                        .unwrap()
                        .id,
                    transaction_category: self
                        .categories
                        .get(self.form_selectected_category.unwrap())
                        .unwrap()
                        .id,
                    description: self.form_note.clone(),
                    amount: self.new_transaction_amount,
                    transaction_date: NaiveDateTime::from_timestamp(self.form_date, 0),
                    is_expense,
                };
                let _ = store.create_money_transaction(&new_transaction);
                commands.push(Task::perform(async {}, |_| AppMessage::UpdateAllPages));
                self.add_transaction_view = false;
                self.form_amount = "".to_string();
                self.form_note = "".to_string();
                self.form_selectected_category = Some(0);
                self.form_selected_bank_account = Some(0);
            }
            TransactionMessage::CandellAddTransaction => {
                self.add_transaction_view = false;
                self.form_amount = "".to_string();
                self.form_note = "".to_string();
                self.form_selectected_category = Some(0);
                self.form_selected_bank_account = Some(0);
            }
            TransactionMessage::FormDateChanged(date) => {
                log::info!("form date changed: {:?}", date);
                self.form_date = date;
            }
            TransactionMessage::PreviousMonth => {
                if self.view_month == 1 {
                    self.view_month = 12;
                    self.view_year -= 1;
                } else {
                    self.view_month -= 1;
                }
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Transactions(TransactionMessage::UpdatePage)
                }));
            }
            TransactionMessage::NextMonth => {
                if self.view_month == 12 {
                    self.view_month = 1;
                    self.view_year += 1;
                } else {
                    self.view_month += 1;
                }
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Transactions(TransactionMessage::UpdatePage)
                }));
            }
            TransactionMessage::ViewChanged(entity) => {
                self.view_selection.activate(entity);
            }
        }
        Task::batch(commands)
    }
}
