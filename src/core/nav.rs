use cosmic::{widget::icon, Element};

use crate::{app::{self, AppMessage}, fl};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Accounts,
    Categories,
    Transactions,
    Settings,
    Welcome,
}

impl Default for &NavPage {
    fn default() -> Self {
        &NavPage::Accounts
    }
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::Accounts => fl!("page_accounts"),
            Self::Categories => fl!("page_categories"),
            Self::Transactions => fl!("page_transactions"),
            Self::Settings => fl!("page_settings"),
            Self::Welcome => fl!("page-welcome"),
        }
    }

    pub fn icon(&self) -> cosmic::widget::Icon {
        match self {
            NavPage::Accounts => icon::from_name("contact-new-symbolic").into(),
            NavPage::Categories => icon::from_name("sidebar-places-symbolic").into(),
            NavPage::Transactions => icon::from_name("network-transmit-receive-symbolic").into(),
            NavPage::Settings => icon::from_name("application-default-symbolic").into(),
            NavPage::Welcome => icon::from_name("application-default-symbolic").into(), //TODO here the icon is useless
        }
    }

    pub fn view<'a>(&self, app: &'a app::MoneyManager) -> Element<'a, AppMessage> {
        match self {
            NavPage::Accounts => app.accounts.view().map(AppMessage::Accounts),
            NavPage::Categories => app.categories.view().map(AppMessage::Categories),
            NavPage::Transactions => app.transactions.view().map(AppMessage::Transactions),
            NavPage::Settings => app.settings.view().map(AppMessage::Settings),
            NavPage::Welcome => app.welcome.view().map(AppMessage::Welcome),
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Accounts,
            Self::Categories,
            Self::Transactions,
            Self::Settings,
        ]
    }
}
