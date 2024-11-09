// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::config::Config;
use crate::core::nav::NavPage;
use crate::synchronization::export::export_to_folder;
use crate::synchronization::import::import_from_json;
use crate::{fl, pages};
use cosmic::app::{self, Core, Task};
use cosmic::dialog::ashpd::url::Url;
use cosmic::dialog::file_chooser::{self, FileFilter};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, menu, nav_bar, ToastId};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};
use futures_util::FutureExt;

pub const QUALIFIER: &str = "com";
pub const ORG: &str = "francescogaglione";
pub const APP: &str = "cosmicmoney";
pub const APPID: &str = constcat::concat!(QUALIFIER, ".", ORG, ".", APP);

pub struct MoneyManager {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    /// A model that contains all of the pages assigned to the nav bar panel.
    nav: nav_bar::Model,

    pub accounts: pages::accounts::Accounts,
    pub categories: pages::categories::Categories,
    pub settings: pages::settings::Settings,
    pub transactions: pages::transactions::Transactions,
    pub statistics: pages::statistics::Statistics,
    pub welcome: pages::welcome::Welcome,
    pub toasts: widget::toaster::Toasts<AppMessage>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),

    Accounts(pages::accounts::AccountsMessage),
    Categories(pages::categories::CategoriesMessage),
    Transactions(pages::transactions::TransactionMessage),
    Settings(pages::settings::SettingsMessage),
    Statistics(pages::statistics::StatisticsMessage),
    Welcome(pages::welcome::WelcomeMessage),

    GoToAccounts,
    ShowToast(String),
    CloseToast(ToastId),
    UpdateAllPages,

    Import,
    Export,
    ImportFromJsonFile(Url),
    ExportToFolder(Url),
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = AppMessage;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => AppMessage::ToggleContextPage(ContextPage::About),
        }
    }
}

impl Application for MoneyManager {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = AppMessage;
    const APP_ID: &'static str = "com.francescogaglione.cosmicmoney";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn init(mut core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut nav = nav_bar::Model::default();

        for &nav_page in NavPage::all() {
            let id = nav
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<NavPage>(nav_page)
                .id();

            if nav_page == NavPage::default() {
                nav.activate(id);
            }
        }

        let config = Config::load();
        if !config.1.is_user_initialized {
            core.nav_bar_set_toggled(false);
        }

        let mut app = MoneyManager {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            nav,
            accounts: pages::accounts::Accounts::default(),
            categories: pages::categories::Categories::default(),
            settings: pages::settings::Settings::default(),
            transactions: pages::transactions::Transactions::default(),
            statistics: pages::statistics::Statistics::default(),
            welcome: pages::welcome::Welcome::default(),
            toasts: widget::toaster::Toasts::new(AppMessage::CloseToast),
        };

        let command = app.update_title();

        (app, command)
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    fn view(&self) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let entity = self.nav.active();
        let nav_page = self.nav.data::<NavPage>(entity).unwrap_or_default();
        let config = Config::load();

        widget::column::with_children(vec![if !config.1.is_user_initialized {
            self.welcome.view().map(AppMessage::Welcome)
        } else {
            nav_page.view(self)
        }])
        .push(widget::toaster(&self.toasts, widget::horizontal_space()))
        .padding(spacing.space_xs)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> cosmic::iced::Task<app::Message<Self::Message>> {
        let mut commands = vec![];
        match message {
            AppMessage::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }
            AppMessage::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            AppMessage::Accounts(message) => {
                commands.push(self.accounts.update(message).map(cosmic::app::Message::App))
            }
            AppMessage::Settings(message) => {
                commands.push(self.settings.update(message).map(cosmic::app::Message::App))
            }
            AppMessage::Categories(message) => commands.push(
                self.categories
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            AppMessage::Transactions(message) => commands.push(
                self.transactions
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            AppMessage::Statistics(message) => commands.push(
                self.statistics
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            AppMessage::Welcome(welcome_message) => {
                commands.push(
                    self.welcome
                        .update(welcome_message)
                        .map(cosmic::app::Message::App),
                );
            }
            AppMessage::GoToAccounts => {
                self.nav.activate_position(0);
                self.core.nav_bar_set_toggled(true);
            }
            AppMessage::ShowToast(message) => {
                commands.push(
                    self.toasts
                        .push(widget::toaster::Toast::new(message))
                        .map(cosmic::app::Message::App),
                );
            }
            AppMessage::CloseToast(id) => {
                self.toasts.remove(id);
            }
            AppMessage::UpdateAllPages => {
                commands.push(
                    self.accounts
                        .update(pages::accounts::AccountsMessage::Update)
                        .map(cosmic::app::Message::App),
                );
                commands.push(
                    self.categories
                        .update(pages::categories::CategoriesMessage::Update)
                        .map(cosmic::app::Message::App),
                );
                commands.push(
                    self.settings
                        .update(pages::settings::SettingsMessage::Update)
                        .map(cosmic::app::Message::App),
                );
                commands.push(
                    self.transactions
                        .update(pages::transactions::TransactionMessage::UpdatePage)
                        .map(cosmic::app::Message::App),
                );
                commands.push(
                    self.statistics
                        .update(pages::statistics::StatisticsMessage::Update)
                        .map(cosmic::app::Message::App),
                );
            }
            AppMessage::Import => {
                commands.push(cosmic::command::future(
                    async move {
                        let filter = FileFilter::new("json files").glob("*.json");
                        let dialog = file_chooser::open::Dialog::new()
                            .title("Choose a data file")
                            .filter(filter);
                        match dialog.open_file().await {
                            Ok(selected_file) => {
                                AppMessage::ImportFromJsonFile(selected_file.url().clone())
                            }
                            Err(file_chooser::Error::Cancelled) => {
                                AppMessage::ShowToast(fl!("operation-cancelled"))
                            }
                            Err(_why) => AppMessage::ShowToast(fl!("operation-cancelled")),
                        }
                    }
                    .map(cosmic::app::Message::App),
                ));
            }
            AppMessage::ImportFromJsonFile(url) => {
                match import_from_json(&url) {
                    Ok(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            cosmic::app::Message::App(AppMessage::ShowToast(fl!("import-success")))
                        }));
                    }
                    Err(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            cosmic::app::Message::App(AppMessage::ShowToast(fl!("import-error")))
                        }));
                    }
                }
                commands.push(
                    self.welcome
                        .update(pages::welcome::WelcomeMessage::ImportCompleted)
                        .map(cosmic::app::Message::App),
                );
                commands.push(Task::perform(async {}, |_| {
                    cosmic::app::Message::App(AppMessage::UpdateAllPages)
                }));
            }
            AppMessage::Export => {
                commands.push(cosmic::command::future(async move {
                    let dialog =
                        file_chooser::open::Dialog::new().title("Choose a destination folder");
                    match dialog.open_folder().await {
                        Ok(selected_folder) => {
                            AppMessage::ExportToFolder(selected_folder.url().clone())
                        }
                        Err(file_chooser::Error::Cancelled) => {
                            AppMessage::ShowToast(fl!("operation-cancelled"))
                        }
                        Err(_why) => AppMessage::ShowToast(fl!("operation-cancelled")),
                    }
                }));
            }
            AppMessage::ExportToFolder(url) => {
                log::info!("Exporting data...");
                match export_to_folder(url) {
                    Ok(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            cosmic::app::Message::App(AppMessage::ShowToast(fl!(
                                "export-completed"
                            )))
                        }));
                    }
                    Err(_) => {
                        commands.push(Task::perform(async {}, |_| {
                            cosmic::app::Message::App(AppMessage::ShowToast(fl!("export-error")))
                        }));
                    }
                }
            }
        }
        Task::batch(commands)
    }

    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        self.update_title()
    }
}

impl MoneyManager {
    pub fn about(&self) -> Element<AppMessage> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::image(widget::image::Handle::from_bytes(
            &include_bytes!(
                "../res/icons/hicolor/128x128/apps/com.francescogaglione.cosmicmoney.png"
            )[..],
        ))
        .height(Length::Fixed(128.))
        .height(Length::Fixed(128.));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link("Home")
            .on_press(AppMessage::LaunchUrl(
                "https://github.com/francesco-gaglione/cosmic-money".to_string(),
            ))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .width(Length::Fill)
            .into()
    }

    pub fn update_title(&mut self) -> Task<AppMessage> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}
