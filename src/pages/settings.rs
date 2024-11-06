use crate::{app::AppMessage, config::Config, fl, models::Currency, STORE};
use cosmic::{
    iced::Length,
    widget::{self, Space},
    Element, Task,
};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Update,
    CurrencyChanged(usize),
    Import,
    Export,
}

pub struct Settings {
    currency_list: Vec<Currency>,
    selected_currency: Option<usize>,
}

impl Default for Settings {
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
        }
    }
}

impl Settings {
    pub fn view<'a>(&'a self) -> Element<'a, SettingsMessage> {
        let mut settings_col = widget::column().width(Length::Fill);

        settings_col = settings_col
            .push(widget::text::title1(fl!("page_settings")))
            .push(Space::with_height(10));

        settings_col = settings_col
            .push(widget::text::title4(fl!("currency")))
            .push(widget::dropdown(
                &self.currency_list,
                self.selected_currency,
                SettingsMessage::CurrencyChanged,
            ));

        settings_col = settings_col
            .push(Space::with_height(20))
            .push(widget::text::title4(fl!("import-export")))
            .push(widget::text::text(fl!("import-export-desc")))
            .push(Space::with_height(5))
            .push(
                widget::row()
                    .push(
                        widget::button::text(fl!("import"))
                            .on_press(SettingsMessage::Import)
                            .class(widget::button::ButtonClass::Suggested),
                    )
                    .push(Space::with_width(10))
                    .push(
                        widget::button::text(fl!("export"))
                            .on_press(SettingsMessage::Export)
                            .class(widget::button::ButtonClass::Suggested),
                    ),
            );

        let main_container = widget::container(settings_col);

        widget::scrollable(main_container).into()
    }

    pub fn update(&mut self, message: SettingsMessage) -> Task<AppMessage> {
        let mut commands = vec![];
        match message {
            SettingsMessage::CurrencyChanged(index) => {
                self.selected_currency = Some(index);
                if let Some(selected_currency) = self.currency_list.get(index).clone() {
                    let mut config = Config::load();
                    let _ = config
                        .1
                        .set_currency_id(&config.0.unwrap(), selected_currency.id);
                }
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Accounts(super::accounts::AccountsMessage::Update)
                }));
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Categories(super::categories::CategoriesMessage::Update)
                }));
                commands.push(Task::perform(async {}, |_| {
                    AppMessage::Transactions(super::transactions::TransactionMessage::UpdatePage)
                }));
            }
            SettingsMessage::Update => {
                let mut store = STORE.lock().unwrap();
                let currencies = store.get_currencies().unwrap_or_else(|_| vec![]);
                let config = Config::load();

                let selected_currency_id = config.1.currency_id;

                let selected_currency = currencies
                    .iter()
                    .position(|currency| currency.id == selected_currency_id)
                    .unwrap_or(0);

                self.selected_currency = Some(selected_currency);
            }
            SettingsMessage::Import => {
                commands.push(Task::perform(async {}, |_| AppMessage::Import));
            }
            SettingsMessage::Export => {
                commands.push(Task::perform(async {}, |_| AppMessage::Export));
            }
        }
        Task::batch(commands)
    }
}
