use cosmic::Element;

use crate::{app, fl, pages};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Accounts,
    Categories,
    Transactions,
    Settings,
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
        }
    }

    pub fn view<'a>(&self, app: &'a app::MoneyManager) -> Element<'a, app::Message> {
        match self {
            NavPage::Accounts => app.accounts.view().map(app::Message::Accounts),
            NavPage::Categories => app.categories.view().map(app::Message::Categories),
            NavPage::Transactions => app.transactions.view().map(app::Message::Transactions),
            NavPage::Settings => app.settings.view().map(app::Message::Settings),
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
