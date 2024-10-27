// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Mutex;

use app::MoneyManager;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use store::Store;
/// The `app` module is used by convention to indicate the main component of our application.
mod app;
mod config;
mod core;
mod errors;
mod models;
mod pages;
mod schema;
mod store;
mod widget;

static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::default()));
static DATABASE_URL: &str = "cosmic-money.db";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    dotenv().ok();
    env_logger::init();

    log::info!("Check db");
    let directories = directories::ProjectDirs::from(app::QUALIFIER, app::ORG, app::APP);
    match directories {
        Some(dirs) => {
            let data_dir = dirs.data_dir();
            let db_path = data_dir.join(DATABASE_URL);
            if !&db_path.exists() {
                match std::fs::create_dir_all(db_path) {
                    Ok(_) => {
                        log::info!("created");
                    }
                    Err(e) => {
                        log::error!("Error creating data file: {:?}", e);
                        panic!("Error creating data file: {:?}", e);
                    }
                }
            }
        }
        None => {
            log::error!("Failed to get app data dir");
            panic!("Failed to get app data dir");
        }
    }
    log::info!("Db ok");

    log::info!("Running migration...");
    let mut connection = SqliteConnection::establish(DATABASE_URL)
        .unwrap_or_else(|e| panic!("Error connecting to {} with error: {:?}", DATABASE_URL, e));
    run_migration(&mut connection);
    let applied = connection.applied_migrations();
    log::info!("Migration completed, applied: {:?}", applied);

    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<MoneyManager>(settings, ())
}
