use cosmic::Element;

use crate::{app, fl, pages};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Accounts,
    Categories,
    Transactions,
    Statistics,
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
            Self::Statistics => fl!("page_stats"),
            Self::Settings => fl!("page_settings"),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, app::Message> {
        match self {
            NavPage::Accounts => pages::accounts::Accounts::default()
                .view()
                .map(app::Message::Accounts),
            NavPage::Categories => pages::categories::Categories::default()
                .view()
                .map(app::Message::Categories),
            NavPage::Transactions => pages::transactions::Transactions::default()
                .view()
                .map(app::Message::Transactions),
            NavPage::Statistics => pages::statistics::Statistics::default()
                .view()
                .map(app::Message::Statistics),
            NavPage::Settings => pages::settings::Settings::default()
                .view()
                .map(app::Message::Settings),
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Accounts,
            Self::Categories,
            Self::Transactions,
            Self::Statistics,
            Self::Settings,
        ]
    }
}
