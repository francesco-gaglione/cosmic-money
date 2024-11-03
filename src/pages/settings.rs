use std::{
    fs::File,
    io::{BufReader, Write},
};

use crate::{
    app,
    config::Config,
    fl,
    models::Currency,
    synchronization::{import::import_from_json, model::SyncModel},
    STORE,
};
use cosmic::{
    dialog::{
        ashpd::url::Url,
        file_chooser::{self, FileFilter},
    },
    iced::Length,
    widget::{self, Space},
    Element, Task,
};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Update,
    CurrencyChanged(usize),
    Import,
    ImportFromJsonFile(Url),
    Export,
    ExportToFolder(Url),
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
            SettingsMessage::Import => {
                commands.push(cosmic::command::future(async move {
                    let filter = FileFilter::new("json files").glob("*.json");
                    let dialog = file_chooser::open::Dialog::new()
                        .title("Choose a data file")
                        .filter(filter);
                    match dialog.open_file().await {
                        Ok(selected_file) => {
                            app::Message::ImportFromFile(selected_file.url().clone())
                        }
                        Err(file_chooser::Error::Cancelled) => {
                            app::Message::ShowToast(fl!("operation-cancelled"))
                        }
                        Err(_why) => app::Message::ShowToast(fl!("operation-cancelled")),
                    }
                }));
            }
            SettingsMessage::ImportFromJsonFile(url) => {
                log::info!("Import from {:?}", url);
                match import_from_json(&url) {
                    Ok(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            app::Message::ShowToast(fl!("import-success"))
                        }));
                    }
                    Err(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            app::Message::ShowToast(fl!("import-error"))
                        }));
                    }
                }
                commands.push(Task::perform(async {}, |_| app::Message::UpdateAllPages));
            }
            SettingsMessage::Export => {
                commands.push(cosmic::command::future(async move {
                    let dialog =
                        file_chooser::open::Dialog::new().title("Choose a destination folder");
                    match dialog.open_folder().await {
                        Ok(selected_folder) => {
                            app::Message::ExportDirectoryChosen(selected_folder.url().clone())
                        }
                        Err(file_chooser::Error::Cancelled) => {
                            app::Message::ShowToast(fl!("operation-cancelled"))
                        }
                        Err(_why) => app::Message::ShowToast(fl!("operation-cancelled")),
                    }
                }));
            }
            SettingsMessage::ExportToFolder(url) => {
                log::info!("Exporting data");
                let mut store = STORE.lock().unwrap();
                let config = Config::load();
                let accounts = store.get_accounts();
                let categories = store.get_categories();
                let transactions = store.get_money_transactions();
                let currencies = store.get_currencies();

                let currency = if let Ok(currencies) = currencies {
                    match currencies.iter().find(|c| c.id == config.1.currency_id) {
                        Some(currency) => currency.symbol.clone(),
                        None => "USD".to_string(),
                    }
                } else {
                    "USD".to_string()
                };

                let sync_model = SyncModel {
                    accounts: accounts.unwrap_or(vec![]),
                    categories: categories.unwrap_or(vec![]),
                    transactions: transactions.unwrap_or(vec![]),
                    currency,
                };

                match serde_json::to_string(&sync_model) {
                    Ok(serialized) => {
                        if let Ok(path) = url.to_file_path() {
                            let target = path.join("exported.json");
                            match File::create(&target)
                                .and_then(|mut file| file.write_all(serialized.as_bytes()))
                            {
                                Ok(_) => {
                                    log::info!("file exported");
                                    commands.push(Task::perform(async {}, |_| {
                                        app::Message::ShowToast(fl!("export-completed"))
                                    }));
                                }
                                Err(_) => {
                                    commands.push(Task::perform(async {}, |_| {
                                        app::Message::ShowToast(fl!("export-error"))
                                    }));
                                }
                            }
                        } else {
                            commands.push(Task::perform(async {}, |_| {
                                app::Message::ShowToast(fl!("export-error"))
                            }));
                        }
                    }
                    Err(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            app::Message::ShowToast(fl!("export-error"))
                        }));
                    }
                }
            }
        }
        Task::batch(commands)
    }
}
