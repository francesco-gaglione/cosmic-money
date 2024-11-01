use crate::models::NewCategory;
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
    DeleteCategory(NewCategory, bool),
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
                widget::column().push(widget::text::title4(fl!("welcome-initial-accounts"))),
            )
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(cosmic::theme::Container::Card),
        );

        main_col = main_col.push(Space::with_height(10));
        main_col = main_col.push(widget::row().push(
            widget::button::text(fl!("setup")).class(widget::button::ButtonClass::Suggested),
        ));

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
                        widget::button::text(fl!("add"))
                            .class(widget::button::ButtonClass::Suggested)
                            .on_press(WelcomeMessage::AddCategoryToggle(is_income)),
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

    pub fn update(&mut self, message: WelcomeMessage) -> Task<crate::app::Message> {
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
        }
        Task::batch(commands)
    }
}
