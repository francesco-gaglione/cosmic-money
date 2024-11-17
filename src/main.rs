// SPDX-License-Identifier: GPL-3.0-only

use core::localization;
use std::{path::PathBuf, sync::Mutex};

use app::MoneyManager;
use cosmic::iced::Size;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use store::Store;

mod app;
mod config;
mod core;
mod errors;
mod models;
mod pages;
mod schema;
mod store;
mod synchronization;
mod utils;
mod widget;

static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::default()));
static DATABASE_URL: &str = "cosmic-money.db";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

use std::fs::File;

pub fn get_database_url() -> PathBuf {
    let directories = ProjectDirs::from(app::QUALIFIER, app::ORG, app::APP)
        .expect("Failed to get app data directory");

    let data_dir = directories.data_dir();
    let db_path = data_dir.join(DATABASE_URL);

    std::fs::create_dir_all(data_dir)
        .unwrap_or_else(|e| panic!("Error creating data directory: {:?}", e));

    if !db_path.exists() {
        File::create(&db_path).unwrap_or_else(|e| panic!("Error creating database file: {:?}", e));
        log::info!("Database file created: {:?}", db_path);
    } else {
        log::info!("Database file already exists: {:?}", db_path);
    }

    log::info!("Returning database path: {:?}", db_path);
    db_path
}

/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    dotenv().ok();
    env_logger::init();

    let database_url_path = get_database_url();
    let database_url = if let Some(url) = database_url_path.to_str() {
        log::info!("url: {}", url);
        url
    } else {
        panic!("unable to get local database url");
    };

    log::info!("Running migration...");
    let mut connection = SqliteConnection::establish(database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {:?} with error: {:?}", database_url, e));
    run_migration(&mut connection);
    let applied = connection.applied_migrations();
    log::info!("Migration completed, applied: {:?}", applied);

    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    localization::init(&requested_languages);

    let settings = cosmic::app::Settings::default()
        .size(Size::new(1200., 1000.))
        .theme(cosmic::theme::system_preference())
        .debug(false);

    cosmic::app::run::<MoneyManager>(settings, ())
}
