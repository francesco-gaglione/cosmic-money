use std::collections::HashMap;

use chrono::{Datelike, Local, NaiveDate};
use cosmic::{
    iced::{
        self,
        alignment::{Horizontal, Vertical},
        Alignment, Length, Padding,
    },
    widget::{self, column, progress_bar, Space},
    Element, Task,
};

use crate::{app::AppMessage, fl, utils::dates::get_month_date_range, STORE};

#[derive(Debug, Clone)]
pub enum StatisticsMessage {
    Update,
    PreviousMonth,
    NextMonth,
}

pub struct Statistics {
    view_month: u32,
    view_year: i32,
    distribution: HashMap<NaiveDate, f32>,
}

impl Default for Statistics {
    fn default() -> Self {
        let now = Local::now();
        let mut statistics = Self {
            view_month: now.month(),
            view_year: now.year(),
            distribution: HashMap::new(),
        };
        statistics.generate_distribution();

        statistics
    }
}

impl Statistics {
    pub fn view<'a>(&'a self) -> Element<'a, StatisticsMessage> {
        let mut element =
            column::<StatisticsMessage>().push(widget::text::title1(fl!("statistics")));

        let month_names = vec![
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
        ];

        element = element.push(
            widget::column()
                .push(
                    widget::row()
                        .push(
                            widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                                .on_press(StatisticsMessage::PreviousMonth),
                        )
                        .push(Space::with_width(10))
                        .push(
                            widget::container(
                                widget::row()
                                    .push(widget::text::text(
                                        month_names[self.view_month as usize - 1].clone(),
                                    ))
                                    .push(Space::with_width(10))
                                    .push(widget::text::text(self.view_year.to_string())),
                            )
                            .padding(Padding::from(7)),
                        )
                        .push(Space::with_width(10))
                        .push(
                            widget::button::icon(widget::icon::from_name("go-next-symbolic"))
                                .on_press(StatisticsMessage::NextMonth),
                        ),
                )
                .align_x(Alignment::Center)
                .width(Length::Fill),
        );

        element = element.push(widget::text::title4(fl!("ratio")));
        element = element.push(widget::text::text(fl!(
            "ratio-value",
            ratio = format!("{:.2}", self.calculate_ratio())
        )));

        element = element.push(Space::with_height(10));
        element = element.push(widget::text::title4(fl!("monthly-distribution")));
        element = element.push(Space::with_height(10));

        if self.distribution.is_empty() {
            element = element.push(widget::text::text(fl!("no-element-distribution")));
        } else {
            element = element
                .push(
                    widget::row()
                        .push(
                            widget::column()
                                .spacing(5)
                                .align_x(Alignment::Center)
                                .push(widget::text::text("Giorno").size(16)),
                        )
                        .push(
                            widget::column().spacing(5).align_x(Alignment::Center).push(
                                widget::text::text("%")
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Right)
                                    .size(16),
                            ),
                        ),
                )
                .push(Space::with_height(10));

            let mut sorted_daily_totals: Vec<(NaiveDate, f32)> =
                self.distribution.clone().into_iter().collect();

            sorted_daily_totals.sort_by_key(|&(date, _)| date);

            for item in sorted_daily_totals {
                element = element
                    .push(
                        widget::row()
                            .spacing(15)
                            .width(Length::Fill)
                            .height(Length::from(40))
                            .push(
                                widget::column()
                                    .width(Length::Fixed(30.))
                                    .spacing(5)
                                    .align_x(Alignment::Center)
                                    .push(
                                        widget::text::text((item.0.clone().day0() + 1).to_string())
                                            .align_y(Vertical::Top)
                                            .size(14),
                                    ),
                            )
                            .push(widget::Space::with_width(Length::Fixed(20.0)))
                            .push(
                                widget::column()
                                    .push(
                                        progress_bar(0.0..=100.0, item.1.clone())
                                            .height(Length::Fixed(10.0)),
                                    )
                                    .push(
                                        widget::text::text(format!("{:.0}%", item.1))
                                            .width(Length::Fill)
                                            .align_x(Horizontal::Right)
                                            .size(14),
                                    ),
                            ),
                    )
                    .push(Space::with_height(5));
            }
        }

        widget::scrollable(
            widget::container(element)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink),
        )
        .into()
    }

    pub fn update(&mut self, message: StatisticsMessage) -> Task<AppMessage> {
        let mut commands = Vec::new();
        match message {
            StatisticsMessage::Update => {
                self.generate_distribution();
            }
            StatisticsMessage::PreviousMonth => {
                if self.view_month == 1 {
                    self.view_month = 12;
                    self.view_year -= 1;
                } else {
                    self.view_month -= 1;
                }
                self.generate_distribution();
            }
            StatisticsMessage::NextMonth => {
                if self.view_month == 12 {
                    self.view_month = 1;
                    self.view_year += 1;
                } else {
                    self.view_month += 1;
                }
                self.generate_distribution();
            }
        }
        Task::batch(commands)
    }

    fn calculate_ratio(&self) -> f32 {
        let mut store = STORE.lock().unwrap();
        let (start_date, end_date) = get_month_date_range(self.view_year, self.view_month);
        let transactions = store.get_money_transactions_date_range(&start_date, &end_date);
        match transactions {
            Ok(transactions) => {
                let income_sum: f32 = transactions
                    .iter()
                    .filter(|t| !t.is_expense)
                    .map(|t| t.amount)
                    .sum();

                let expense_sum: f32 = transactions
                    .iter()
                    .filter(|t| t.is_expense)
                    .map(|t| t.amount)
                    .sum();

                income_sum / expense_sum
            }
            Err(_) => 0.,
        }
    }

    pub fn generate_distribution(&mut self) {
        let mut store = STORE.lock().unwrap();
        let (start_date, end_date) = get_month_date_range(self.view_year, self.view_month);
        let transactions = store.get_money_transactions_date_range(&start_date, &end_date);

        let mut daily_totals: HashMap<NaiveDate, f32> = HashMap::new();

        match transactions {
            Ok(transactions) => {
                for transaction in &transactions {
                    if transaction.is_expense {
                        let date = transaction.transaction_date.date();
                        *daily_totals.entry(date).or_insert(0.0) += transaction.amount;
                    }
                }

                let total_spending: f32 = daily_totals.values().sum();

                self.distribution = daily_totals
                    .into_iter()
                    .map(|(date, total)| {
                        let percentage = if total_spending > 0.0 {
                            (total / total_spending) * 100.0
                        } else {
                            0.0
                        };
                        (date, percentage)
                    })
                    .collect();
            }
            Err(_) => {
                self.distribution = HashMap::new();
            }
        }
    }
}
