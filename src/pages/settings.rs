use crate::{app, config::Config, fl, models::Currency, synchronization::model::SyncModel, STORE};
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
            //TODO create a progress spinner to track exporting process
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

    pub fn update(&mut self, message: SettingsMessage) -> Task<crate::app::Message> {
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
                    app::Message::Accounts(super::accounts::AccountsMessage::Update)
                }));
                commands.push(Task::perform(async {}, |_| {
                    app::Message::Categories(super::categories::CategoriesMessage::Update)
                }));
                commands.push(Task::perform(async {}, |_| {
                    app::Message::Transactions(super::transactions::TransactionMessage::UpdatePage)
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
            SettingsMessage::Import => todo!(),
            SettingsMessage::Export => {
                commands.push(cosmic::command::future(async move {
                    log::info!("opening new dialog");
                    
                    let dialog =
                        file_chooser::open::Dialog::new().title("Choose a destination folder");

                    match dialog.open_file().await {
                        Ok(response) => {
                            //InstallFromFileMessage::FileSelected(response.url().to_owned())

                            app::Message::ChooseFile(response.url().clone())
                        }

                        Err(file_chooser::Error::Cancelled) => app::Message::Cancelled,

                        Err(why) => app::Message::OpenError(Arc::new(why)),
                    }
                }));

                log::info!("Exporting data");
                let mut store = STORE.lock().unwrap();
                let accounts = store.get_accounts();
                let categories = store.get_categories();
                let transactions = store.get_money_transactions();

                let sync_model = SyncModel {
                    accounts: accounts.unwrap_or(vec![]),
                    categories: categories.unwrap_or(vec![]),
                    transactions: transactions.unwrap_or(vec![]),
                    currency: "".to_string(),
                };

                log::debug!("exporting data: {:?}", sync_model);
            }
        }
        Task::batch(commands)
    }
}
