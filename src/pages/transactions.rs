use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use cosmic::{
    iced::{Alignment, Length, Padding},
    widget::{self, column, horizontal_space, text_input, vertical_space},
    Command, Element,
};

use crate::{
    app, fl,
    models::{Account, Category, MoneyTransaction, NewMoneyTransaction},
    widget::date_picker::date_picker,
    STORE,
};

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
}

pub struct Transactions {
    add_transaction_view: bool,
    categories: Vec<Category>,
    accounts: Vec<Account>,
    form_transaction_type: widget::segmented_button::SingleSelectModel,
    form_selectected_category: Option<usize>,
    transactions: Vec<MoneyTransaction>,
    form_selected_bank_account: Option<usize>,
    form_note: String,
    form_amount: String,
    form_date: i64,
    new_transaction_amount: f32,
}

impl Default for Transactions {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let transactions = store.get_money_transactions();
        Self {
            add_transaction_view: false,
            categories: store.get_categories().unwrap_or_else(|_| vec![]),
            accounts: store.get_accounts().unwrap_or_else(|_| vec![]),
            form_transaction_type: widget::segmented_button::Model::builder()
                .insert(|b| b.text(fl!("expense")).data(1u16).activate())
                .insert(|b| b.text(fl!("income")).data(2u16))
                .build(),
            form_note: "".to_string(),
            form_selectected_category: Some(0),
            form_selected_bank_account: Some(0),
            transactions: transactions.unwrap_or_else(|_| vec![]),
            form_amount: "".to_string(),
            form_date: Utc::now().timestamp(),
            new_transaction_amount: 0.,
        }
    }
}

impl Transactions {
    pub fn view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        if self.add_transaction_view {
            self.new_transaction_view()
        } else {
            self.transactions_view()
        }
    }

    pub fn transactions_view<'a>(&self) -> Element<'a, TransactionMessage> {
        let mut element = widget::column()
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
                                    .style(widget::button::Style::Suggested),
                            )
                            .width(Length::Fill)
                            .align_items(Alignment::End),
                    ),
            )
            .width(Length::Fill)
            .align_items(Alignment::Start);

        if self.transactions.len() > 0 {
            for t in &self.transactions {
                element = element
                    .push(
                        widget::container(
                            widget::column()
                                .push(
                                    widget::row()
                                        .push(
                                            widget::text::text(format!(
                                                "{}: {}",
                                                fl!("amount"),
                                                t.amount
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
                                .push(vertical_space(Length::from(5)))
                                .push(widget::row().push(widget::text::text(format!(
                                    "{}: {}",
                                    fl!("note"),
                                    t.description
                                ))))
                                .width(Length::Fill),
                        )
                        .width(Length::Fill)
                        .padding(Padding::new(10.))
                        .style(cosmic::theme::Container::Card),
                    )
                    .push(vertical_space(Length::from(10)))
            }
        } else {
            element = element.push(widget::text::text(fl!("no-elements")))
        }

        element.into()
    }

    pub fn new_transaction_view<'a>(&'a self) -> Element<'a, TransactionMessage> {
        let mut element = widget::column().width(Length::Fill);

        element = element.push(widget::text::title1(fl!("add-transaction")));

        element = element.push(widget::vertical_space(10));

        element = element.push(
            widget::segmented_control::horizontal(&self.form_transaction_type)
                .on_activate(TransactionMessage::FormTransactionTypeChanged),
        );

        element = element.push(widget::vertical_space(10));

        element = element.push(
            widget::column()
                .push(widget::text::text(fl!("amount")))
                .push(
                    text_input(fl!("amount"), &self.form_amount)
                        .width(Length::Fill)
                        .on_input(TransactionMessage::FormAmountChanged),
                ),
        );

        element = element.push(widget::vertical_space(10));

        element = element.push(date_picker(self.form_date, |date| {
            TransactionMessage::FormDateChanged(date)
        }));

        element = element.push(widget::vertical_space(10));

        element = element
            .push(
                widget::row()
                    .push(
                        widget::column()
                            .push(widget::text::text(fl!("category")))
                            .push(widget::vertical_space(5))
                            .push(widget::dropdown(
                                &self.categories,
                                self.form_selectected_category,
                                TransactionMessage::FormCategoryChanged,
                            )),
                    )
                    .push(horizontal_space(Length::from(20)))
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

        element = element.push(vertical_space(Length::from(10))).push(
            widget::row()
                .push(
                    widget::button::text(fl!("add-transaction"))
                        .on_press(TransactionMessage::SubmitTransaction)
                        .style(widget::button::Style::Suggested),
                )
                .push(widget::horizontal_space(Length::from(10)))
                .push(
                    widget::button::text(fl!("cancel"))
                        .on_press(TransactionMessage::CandellAddTransaction)
                        .style(widget::button::Style::Destructive),
                ),
        );

        element.into()
    }

    pub fn update(&mut self, message: TransactionMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            TransactionMessage::UpdatePage => {
                log::info!("updating page");
                let mut store = STORE.lock().unwrap();
                self.transactions = store.get_money_transactions().unwrap_or_else(|_| vec![]);
                self.categories = store.get_categories().unwrap_or_else(|_| vec![]);
                self.accounts = store.get_accounts().unwrap_or_else(|_| vec![]);
            }
            TransactionMessage::AddTransaction => {
                self.add_transaction_view = true;
            }
            TransactionMessage::FormCategoryChanged(selected) => {
                self.form_selectected_category = Some(selected)
            }
            TransactionMessage::FormTransactionTypeChanged(key) => {
                self.form_transaction_type.activate(key);
            }
            TransactionMessage::FormBankAccountChanged(selected) => {
                self.form_selected_bank_account = Some(selected);
            }
            TransactionMessage::FormNoteChanged(note) => {
                self.form_note = note;
            }
            TransactionMessage::FormAmountChanged(new_amount) => match new_amount.parse::<f32>() {
                Ok(parsed_amount) => {
                    self.new_transaction_amount = parsed_amount;
                    self.form_amount = new_amount;
                }
                Err(_) => {
                    eprintln!("Failed to parse the amount: {}", new_amount);
                }
            },
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
                store.create_money_transaction(&new_transaction);
                commands.push(Command::perform(async {}, |_| {
                    app::Message::Transactions(TransactionMessage::UpdatePage)
                }));
                self.add_transaction_view = false;
            }
            TransactionMessage::CandellAddTransaction => {
                self.add_transaction_view = false;
            }
            TransactionMessage::FormDateChanged(date) => {
                log::info!("form date changed: {:?}", date);
                self.form_date = date;
            }
        }
        Command::batch(commands)
    }
}
