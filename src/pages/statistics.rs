use cosmic::{
    iced::{self, Length, Padding},
    widget::{self, column, settings, Space},
    Element, Task,
};

use crate::{
    app::{self, AppMessage},
    config::Config,
    fl,
    models::{Account, NewAccount, UpdateAccount},
    STORE,
};

#[derive(Debug, Clone)]
pub enum StatisticsMessage {
    Update,
}

pub struct Statistics {}

impl Default for Statistics {
    fn default() -> Self {
        Self {}
    }
}

impl Statistics {
    pub fn view<'a>(&'a self) -> Element<'a, StatisticsMessage> {
        let mut col = column::<StatisticsMessage>().push(widget::text::title1(fl!("statistics")));

        widget::scrollable(
            widget::container(col)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink),
        )
        .into()
    }

    pub fn update(&mut self, message: StatisticsMessage) -> Task<AppMessage> {
        let mut commands = Vec::new();
        match message {
            StatisticsMessage::Update => {}
        }
        Task::batch(commands)
    }
}
