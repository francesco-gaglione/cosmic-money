// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Mutex;

use app::MoneyManager;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use store::Store;
/// The `app` module is used by convention to indicate the main component of our application.
mod app;
mod core;
mod pages;
mod store;
mod models;
mod schema;

static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::default()));

/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    dotenv().ok();
    env_logger::init();
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<MoneyManager>(settings, ())
}
