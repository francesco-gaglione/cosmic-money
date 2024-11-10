use crate::app::AppMessage;
use crate::models::{NewAccount, NewCategory};
use crate::{config::Config, fl, models::Currency, STORE};
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::{Alignment, Padding};
use cosmic::{
    iced::Length,
    widget::{self, Space},
    Element, Task,
};

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    CurrencyChanged(usize),
    AddCategoryToggle(bool),
    NewCategoryNameChanged(String),
    NewCategoryDescriptionChanged(String),
    NewCategorySubmitted(bool),
    NewCategoryCancel,
    DeleteCategory(NewCategory, bool),
    AddAccountToggle,
    NewAccountNameChanged(String),
    NewAccountDescriptionChanged(String),
    NewAccountBalanceChanged(String),
    NewAccountSubmitted,
    NewAccountCancel,
    DeleteAccount(NewAccount),
    Setup,
    Import,
    ImportCompleted,
    UseSuggestedCategories(bool),
    UseSuggestedAccounts,
}

pub struct Welcome {
    currency_list: Vec<Currency>,
    selected_currency: Option<usize>,
    income_categories: Vec<NewCategory>,
    expense_categories: Vec<NewCategory>,
    add_income_toogled: bool,
    add_expense_toogled: bool,
    form_new_category_name: String,
    form_new_category_description: String,
    accounts: Vec<NewAccount>,
    add_account_toggled: bool,
    form_new_account_name: String,
    form_new_account_description: String,
    form_new_account_balance: String,
}

impl Default for Welcome {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let currencies = store.get_currencies().unwrap_or_else(|_| vec![]);
        let config = Config::load();

        let selected_currency_id = config.1.currency_id;

        let selected_currency = currencies
            .iter()
            .position(|currency| currency.id == selected_currency_id)
            .unwrap_or(0);

        Self {
            currency_list: currencies,
            selected_currency: Some(selected_currency),
            income_categories: vec![],
            expense_categories: vec![],
            add_income_toogled: false,
            add_expense_toogled: false,
            form_new_category_name: "".to_string(),
            form_new_category_description: "".to_string(),
            accounts: vec![],
            add_account_toggled: false,
            form_new_account_name: "".to_string(),
            form_new_account_description: "".to_string(),
            form_new_account_balance: "0".to_string(),
        }
    }
}

impl Welcome {
    pub fn view<'a>(&'a self) -> Element<'a, WelcomeMessage> {
        let mut main_col = widget::column().width(Length::Fill);

        main_col = main_col.push(
            widget::column()
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .push(widget::text::title1(fl!("welcome-title-message")))
                .push(widget::text::title2(fl!("welcome-message"))),
        );

        main_col = main_col.push(Space::with_height(40));

        main_col = main_col.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("import")))
                    .push(widget::text::text(fl!("welcome-import-message")))
                    .push(widget::Space::with_height(10))
                    .push(
                        widget::button::text(fl!("import"))
                            .on_press(WelcomeMessage::Import)
                            .class(widget::button::ButtonClass::Suggested),
                    )
                    .push(widget::Space::with_height(10)),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("currency")))
                    .push(widget::text::text(fl!("currency-message")))
                    .push(widget::dropdown(
                        &self.currency_list,
                        self.selected_currency,
                        WelcomeMessage::CurrencyChanged,
                    )),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("welcome-income-categories")))
                    .push(self.categories_view(true)),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("welcome-expense-categories")))
                    .push(self.categories_view(false)),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("welcome-initial-accounts")))
                    .push(self.accounts_view()),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(
            widget::row().push(
                widget::button::text(fl!("setup"))
                    .on_press(WelcomeMessage::Setup)
                    .class(widget::button::ButtonClass::Suggested),
            ),
        );

        let main_container = widget::container(main_col);

        widget::scrollable(main_container).into()
    }

    pub fn categories_view<'a>(&'a self, is_income: bool) -> Element<'a, WelcomeMessage> {
        let mut element = widget::column();

        if self.income_categories.is_empty() {
            element = element.push(widget::text::text(fl!("no-elements")))
        }

        let category_list = if is_income {
            &self.income_categories
        } else {
            &self.expense_categories
        };

        for c in category_list {
            element = element.push(Space::with_height(5));
            element = element.push(
                widget::row()
                    .width(Length::Fill)
                    .push(
                        widget::container(widget::text::text(&c.name))
                            .padding(Padding::new(0.).top(6)),
                    )
                    .push(
                        widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
                            .on_press(WelcomeMessage::DeleteCategory(c.clone(), is_income)),
                    ),
            );
            element = element.push(Space::with_height(5));
        }

        element = element.push(Space::with_height(10));
        element = element.push(widget::divider::horizontal::default().width(Length::Fill));
        if (is_income && self.add_income_toogled) || (!is_income && self.add_expense_toogled) {
            element = element.push(self.add_category_view(is_income));
        } else {
            element = element.push(
                widget::column()
                    .push(Space::with_height(5))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("add"))
                                    .class(widget::button::ButtonClass::Suggested)
                                    .on_press(WelcomeMessage::AddCategoryToggle(is_income)),
                            )
                            .push(Space::with_width(10))
                            .push_maybe(
                                match (
                                    is_income,
                                    self.income_categories.is_empty(),
                                    self.expense_categories.is_empty(),
                                ) {
                                    (true, true, _) | (false, _, true) => Some(
                                        widget::button::text(fl!("use-suggested"))
                                            .class(widget::button::ButtonClass::Suggested)
                                            .on_press(WelcomeMessage::UseSuggestedCategories(
                                                is_income,
                                            )),
                                    ),
                                    _ => None,
                                },
                            ),
                    )
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
            );
        }
        element.into()
    }

    pub fn add_category_view<'a>(&'a self, is_income: bool) -> Element<'a, WelcomeMessage> {
        let mut element = widget::column();
        element = element.push(Space::with_height(10));
        element = element.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("new-category")))
                    .push(Space::with_height(10))
                    .push(
                        widget::row().push(
                            widget::column()
                                .push(widget::text::text(fl!("category-name")))
                                .push(Space::with_height(3))
                                .push(
                                    cosmic::widget::text_input(
                                        fl!("new-category"),
                                        &self.form_new_category_name,
                                    )
                                    .on_input(WelcomeMessage::NewCategoryNameChanged),
                                ),
                        ),
                    )
                    .push(Space::with_height(10))
                    .push(
                        widget::row().push(
                            widget::column()
                                .push(widget::text::text(fl!("category-description")))
                                .push(Space::with_height(3))
                                .push(
                                    cosmic::widget::text_input(
                                        fl!("category-description"),
                                        &self.form_new_category_description,
                                    )
                                    .on_input(WelcomeMessage::NewCategoryDescriptionChanged),
                                ),
                        ),
                    )
                    .push(Space::with_height(5))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("add-category"))
                                    .on_press(WelcomeMessage::NewCategorySubmitted(is_income))
                                    .class(widget::button::ButtonClass::Suggested),
                            )
                            .push(Space::with_width(5))
                            .push(
                                widget::button::text(fl!("cancel"))
                                    .on_press(WelcomeMessage::NewCategoryCancel)
                                    .class(widget::button::ButtonClass::Destructive),
                            )
                            .width(Length::Fill)
                            .align_y(Alignment::End),
                    )
                    .width(Length::Fill),
            )
            .padding(10)
            .width(Length::Fill)
            .class(cosmic::theme::Container::Card),
        );

        element = element.push(Space::with_height(10));

        element.into()
    }

    pub fn accounts_view<'a>(&'a self) -> Element<'a, WelcomeMessage> {
        let mut element = widget::column();

        if self.accounts.is_empty() {
            element = element.push(widget::text::text(fl!("no-elements")))
        }

        for a in &self.accounts {
            element = element.push(Space::with_height(5));
            element = element.push(
                widget::row()
                    .width(Length::Fill)
                    .push(
                        widget::container(widget::text::text(&a.name))
                            .padding(Padding::new(0.).top(6)),
                    )
                    .push(
                        widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
                            .on_press(WelcomeMessage::DeleteAccount(a.clone())),
                    ),
            );
            element = element.push(Space::with_height(5));
        }

        element = element.push(Space::with_height(10));
        element = element.push(widget::divider::horizontal::default());

        if self.add_account_toggled {
            element = element.push(self.add_accounts_view());
        } else {
            element = element.push(
                widget::column()
                    .push(Space::with_height(5))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("add"))
                                    .on_press(WelcomeMessage::AddAccountToggle)
                                    .class(widget::button::ButtonClass::Suggested),
                            )
                            .push(Space::with_width(10))
                            .push(
                                widget::button::text(fl!("use-suggested"))
                                    .on_press(WelcomeMessage::UseSuggestedAccounts)
                                    .class(widget::button::ButtonClass::Suggested),
                            ),
                    )
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
            );
        }

        element.into()
    }

    pub fn add_accounts_view<'a>(&'a self) -> Element<'a, WelcomeMessage> {
        let mut element = widget::column().width(Length::Fill);

        element = element.push(Space::with_height(10));

        element = element.push(widget::text::text(fl!("account-name")));
        element = element.push(Space::with_height(3)).push(
            cosmic::widget::text_input(fl!("account-name"), &self.form_new_account_name)
                .on_input(WelcomeMessage::NewAccountNameChanged),
        );

        element = element.push(Space::with_height(5));
        element = element.push(widget::text::text(fl!("account-description")));
        element = element.push(Space::with_height(3)).push(
            cosmic::widget::text_input(
                fl!("account-description"),
                &self.form_new_account_description,
            )
            .on_input(WelcomeMessage::NewAccountDescriptionChanged),
        );

        element = element.push(Space::with_height(5));
        element = element.push(widget::text::text(fl!("balance")));
        element = element.push(Space::with_height(3)).push(
            cosmic::widget::text_input(fl!("balance"), &self.form_new_account_balance)
                .on_input(WelcomeMessage::NewAccountBalanceChanged),
        );

        element = element.push(Space::with_height(5));

        element = element
            .push(
                widget::row()
                    .push(
                        widget::button::text(fl!("add-account"))
                            .on_press(WelcomeMessage::NewAccountSubmitted)
                            .class(widget::button::ButtonClass::Suggested),
                    )
                    .push(Space::with_width(5))
                    .push(
                        widget::button::text(fl!("cancel"))
                            .on_press(WelcomeMessage::NewAccountCancel)
                            .class(widget::button::ButtonClass::Destructive),
                    )
                    .width(Length::Fill)
                    .align_y(Alignment::End),
            )
            .width(Length::Fill);

        element.into()
    }

    pub fn update(&mut self, message: WelcomeMessage) -> Task<AppMessage> {
        let mut commands = vec![];
        match message {
            WelcomeMessage::CurrencyChanged(index) => {
                self.selected_currency = Some(index);
                if let Some(selected_currency) = self.currency_list.get(index).clone() {
                    let mut config = Config::load();
                    let _ = config
                        .1
                        .set_currency_id(&config.0.unwrap(), selected_currency.id);
                }
            }
            WelcomeMessage::AddCategoryToggle(is_income) => {
                if is_income {
                    self.add_income_toogled = true;
                } else {
                    self.add_expense_toogled = true;
                }
            }
            WelcomeMessage::NewCategoryNameChanged(value) => {
                self.form_new_category_name = value;
            }
            WelcomeMessage::NewCategoryDescriptionChanged(value) => {
                self.form_new_category_description = value;
            }
            WelcomeMessage::NewCategorySubmitted(is_income) => {
                if !self.form_new_category_name.is_empty() {
                    let new_category = NewCategory {
                        name: self.form_new_category_name.clone(),
                        is_income,
                        category_description: self.form_new_category_description.clone(),
                    };
                    if is_income {
                        self.income_categories.push(new_category);
                        self.add_income_toogled = false;
                    } else {
                        self.expense_categories.push(new_category);
                        self.add_expense_toogled = false;
                    }
                    self.form_new_category_name = "".to_string();
                    self.form_new_category_description = "".to_string();
                }
            }
            WelcomeMessage::NewCategoryCancel => {
                self.form_new_category_name = "".to_string();
                self.form_new_category_description = "".to_string();
                self.add_income_toogled = false;
                self.add_expense_toogled = false;
            }
            WelcomeMessage::DeleteCategory(delete_category, is_income) => {
                if is_income {
                    self.income_categories
                        .retain(|c| c.name != delete_category.name);
                } else {
                    self.expense_categories
                        .retain(|c| c.name != delete_category.name);
                }
            }
            WelcomeMessage::AddAccountToggle => {
                self.add_account_toggled = true;
            }
            WelcomeMessage::NewAccountNameChanged(name) => {
                self.form_new_account_name = name;
            }
            WelcomeMessage::NewAccountDescriptionChanged(description) => {
                self.form_new_account_description = description;
            }
            WelcomeMessage::NewAccountBalanceChanged(balance) => {
                if balance.parse::<f32>().is_ok() || balance == "" {
                    self.form_new_account_balance = balance;
                }
            }
            WelcomeMessage::NewAccountSubmitted => {
                let balance = if let Ok(balance) = self.form_new_account_balance.parse::<f32>() {
                    balance
                } else {
                    0.
                };

                let new_account = NewAccount {
                    name: self.form_new_account_name.clone(),
                    initial_balance: balance,
                    account_description: self.form_new_account_description.clone(),
                };

                self.accounts.push(new_account);
                self.form_new_account_name = "".to_string();
                self.form_new_account_description = "".to_string();
                self.form_new_account_balance = "0".to_string();
                self.add_account_toggled = false;
            }
            WelcomeMessage::NewAccountCancel => {
                self.form_new_account_name = "".to_string();
                self.form_new_account_description = "".to_string();
                self.form_new_account_balance = "".to_string();
                self.add_account_toggled = false;
            }
            WelcomeMessage::DeleteAccount(delete_account) => {
                self.accounts.retain(|a| a.name != delete_account.name);
            }
            WelcomeMessage::Setup => {
                let mut store = STORE.lock().unwrap();
                let _ = store.create_accounts(&self.accounts);
                let _ = store.create_categories(&self.income_categories);
                let _ = store.create_categories(&self.expense_categories);
                if let Some(selected_currency) = self
                    .currency_list
                    .get(self.selected_currency.unwrap())
                    .clone()
                {
                    let mut config = Config::load();
                    let _ = config
                        .1
                        .set_currency_id(&config.clone().0.unwrap(), selected_currency.id);
                    let _ = config.1.set_is_user_initialized(&config.0.unwrap(), true);
                }
                commands.push(Task::perform(async {}, |_| AppMessage::GoToAccounts));
                commands.push(Task::perform(async {}, |_| AppMessage::UpdateAllPages));
            }
            WelcomeMessage::Import => {
                commands.push(Task::perform(async {}, |_| AppMessage::Import));
            }
            WelcomeMessage::ImportCompleted => {
                let mut config = Config::load();
                let _ = config.1.set_is_user_initialized(&config.0.unwrap(), true);

                commands.push(Task::perform(async {}, |_| AppMessage::GoToAccounts));
                commands.push(Task::perform(async {}, |_| AppMessage::UpdateAllPages));
            }
            WelcomeMessage::UseSuggestedCategories(is_income) => {
                log::info!("use suggested");
                if is_income {
                    self.income_categories.push(NewCategory {
                        name: fl!("salary"),
                        is_income: true,
                        category_description: "".to_string(),
                    });
                    self.income_categories.push(NewCategory {
                        name: fl!("other"),
                        is_income: true,
                        category_description: "".to_string(),
                    });
                } else {
                    self.expense_categories.push(NewCategory {
                        name: fl!("groceries"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("restaurant"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("leisure"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("transport"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("health"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("gifts"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("finance"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("shopping"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("home"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("subscriptions"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                    self.expense_categories.push(NewCategory {
                        name: fl!("travel"),
                        is_income: false,
                        category_description: "".to_string(),
                    });
                }
            }
            WelcomeMessage::UseSuggestedAccounts => {
                self.accounts.push(NewAccount {
                    name: fl!("bank-account"),
                    initial_balance: 0.,
                    account_description: "".to_string(),
                });
                self.accounts.push(NewAccount {
                    name: fl!("cash"),
                    initial_balance: 0.,
                    account_description: "".to_string(),
                });
            }
        }
        Task::batch(commands)
    }
}
