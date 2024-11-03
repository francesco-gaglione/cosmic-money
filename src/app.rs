// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::config::Config;
use crate::core::nav::NavPage;
use crate::{fl, pages};
use cosmic::app::{self, Core, Task};
use cosmic::dialog::ashpd::url::Url;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, menu, nav_bar, ToastId};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};

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
    pub welcome: pages::welcome::Welcome,
    pub toasts: widget::toaster::Toasts<Message>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),

    Accounts(pages::accounts::AccountsMessage),
    Categories(pages::categories::CategoriesMessage),
    Transactions(pages::transactions::TransactionMessage),
    Settings(pages::settings::SettingsMessage),
    Welcome(pages::welcome::WelcomeMessage),

    GoToAccounts,
    ExportDirectoryChosen(Url),
    ImportFromFile(Url),
    ShowToast(String),
    CloseToast(ToastId),
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
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

impl Application for MoneyManager {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
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
            welcome: pages::welcome::Welcome::default(),
            toasts: widget::toaster::Toasts::new(Message::CloseToast),
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
            self.welcome.view().map(Message::Welcome)
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
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }
            Message::ToggleContextPage(context_page) => {
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
            Message::Accounts(message) => {
                commands.push(self.accounts.update(message).map(cosmic::app::Message::App))
            }
            Message::Settings(message) => {
                commands.push(self.settings.update(message).map(cosmic::app::Message::App))
            }
            Message::Categories(message) => commands.push(
                self.categories
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            Message::Transactions(message) => commands.push(
                self.transactions
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            Message::Welcome(welcome_message) => {
                commands.push(
                    self.welcome
                        .update(welcome_message)
                        .map(cosmic::app::Message::App),
                );
            }
            Message::GoToAccounts => {
                self.nav.activate_position(0);
                self.core.nav_bar_set_toggled(true);
            }
            Message::ExportDirectoryChosen(url) => {
                log::info!("Export directory: {:?}", url);
                commands.push(
                    self.settings
                        .update(pages::settings::SettingsMessage::ExportToFolder(url))
                        .map(cosmic::app::Message::App),
                )
            }
            Message::ImportFromFile(url) => commands.push(
                self.settings
                    .update(pages::settings::SettingsMessage::ImportFromJsonFile(url))
                    .map(cosmic::app::Message::App),
            ),
            Message::ShowToast(message) => {
                commands.push(
                    self.toasts
                        .push(widget::toaster::Toast::new(message))
                        .map(cosmic::app::Message::App),
                );
            }
            Message::CloseToast(id) => {
                self.toasts.remove(id);
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
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!(
                "../res/icons/hicolor/128x128/apps/com.francescogaglione.cosmicmoney.png"
            )[..],
        ));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link("Home")
            .on_press(Message::LaunchUrl(
                "https://github.com/francesco-gaglione/cosmic-money".to_string(),
            ))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    pub fn update_title(&mut self) -> Task<Message> {
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
