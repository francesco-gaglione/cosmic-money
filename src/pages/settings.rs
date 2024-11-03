use crate::{
    app,
    config::Config,
    fl,
    models::Currency,
    synchronization::{export::export_to_folder, import::import_from_json},
    STORE,
};
use cosmic::{
    dialog::{
        ashpd::url::Url,
        file_chooser::{self, FileFilter},
    },
    iced::{Length, Padding},
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
    synchronization_progress: f32,
    synchronization_in_progress: bool,
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
            synchronization_progress: 0.,
            synchronization_in_progress: false,
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
            .push_maybe(match self.synchronization_in_progress {
                true => Some(
                    widget::container(
                        widget::progress_bar(0.0..=100.0, self.synchronization_progress)
                            .height(Length::from(5)),
                    )
                    .width(Length::Fill)
                    .padding(Padding::new(5.).vertical()),
                ),
                false => None,
            })
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
                self.synchronization_in_progress = true;
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
                match import_from_json(&url, &mut self.synchronization_progress) {
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
                self.synchronization_in_progress = false;
                commands.push(Task::perform(async {}, |_| app::Message::UpdateAllPages));
            }
            SettingsMessage::Export => {
                self.synchronization_in_progress = true;
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
                log::info!("Exporting data...");
                match export_to_folder(url, &mut self.synchronization_progress) {
                    Ok(_) => {
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
                self.synchronization_in_progress = false;
            }
        }
        Task::batch(commands)
    }
}
