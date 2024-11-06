use chrono::{Datelike, Duration, Local, NaiveDate};
use cosmic::{
    iced::{self, Alignment, Length, Padding},
    widget::{self, column, Space},
    Element, Task,
};

use crate::{
    app::{self, AppMessage},
    config::Config,
    fl, STORE,
};

#[derive(Debug, Clone)]
pub enum StatisticsMessage {
    Update,
    PreviousMonth,
    NextMonth,
}

pub struct Statistics {
    view_month: u32,
    view_year: i32,
}

impl Default for Statistics {
    fn default() -> Self {
        let now = Local::now();
        Self {
            view_month: now.month(),
            view_year: now.year(),
        }
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
            StatisticsMessage::Update => {}
            StatisticsMessage::PreviousMonth => {
                if self.view_month == 1 {
                    self.view_month = 12;
                    self.view_year -= 1;
                } else {
                    self.view_month -= 1;
                }
            }
            StatisticsMessage::NextMonth => {
                if self.view_month == 12 {
                    self.view_month = 1;
                    self.view_year += 1;
                } else {
                    self.view_month += 1;
                }
            }
        }
        Task::batch(commands)
    }

    fn calculate_ratio(&self) -> f32 {
        let mut store = STORE.lock().unwrap();
        let (start_date, end_date) = self.get_month_start_and_end();
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

    fn get_month_start_and_end(&self) -> (NaiveDate, NaiveDate) {
        let month_start = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1)
            .expect("Data non valida per l'inizio del mese");

        let next_month = if self.view_month == 12 {
            NaiveDate::from_ymd_opt((self.view_month + 1) as i32, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(self.view_year, self.view_month + 1, 1)
        }
        .expect("Data non valida per il primo giorno del mese successivo");

        let month_end = next_month - Duration::days(1);

        (month_start, month_end)
    }
}
