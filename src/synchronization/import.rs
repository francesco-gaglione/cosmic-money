use std::{fs::File, io::BufReader};

use cosmic::dialog::ashpd::url::Url;

use crate::{
    config::Config,
    models::{NewAccount, NewCategory, NewMoneyTransaction},
    STORE,
};

use super::model::SyncModel;

pub fn import_from_json(url: &Url, progress: &mut f32) -> Result<(), String> {
    log::info!("Starting import from JSON at URL: {:?}", url);

    match File::open(url.to_file_path().unwrap()) {
        Ok(file) => {
            log::info!("File opened successfully.");
            let reader = BufReader::new(file);

            let deserialized: SyncModel = if let Ok(data) = serde_json::from_reader(reader) {
                log::info!("Deserialization successful.");
                data
            } else {
                log::warn!("Failed to deserialize JSON, using default SyncModel.");
                SyncModel::default()
            };

            *progress = 20.;

            let mut config = Config::load();
            log::info!("Config loaded successfully.");

            let mut store = STORE.lock().unwrap();
            log::info!("STORE lock acquired.");

            let _ = store.drop_all();

            let _ = store.create_accounts(
                &deserialized
                    .accounts
                    .iter()
                    .map(|a| NewAccount::from(a))
                    .collect(),
            );
            log::info!("Accounts imported.");
            *progress = 40.;

            let _ = store.create_categories(
                &deserialized
                    .categories
                    .iter()
                    .map(|c| NewCategory::from(c))
                    .collect(),
            );
            log::info!("Categories imported.");
            *progress = 60.;

            let _ = store.create_money_transactions(
                &deserialized
                    .transactions
                    .iter()
                    .map(|t| NewMoneyTransaction::from(t))
                    .collect(),
            );
            log::info!("Transactions imported.");
            *progress = 80.;

            match store.get_currencies() {
                Ok(list) => {
                    let selected = list.iter().find(|c| c.symbol == deserialized.currency);
                    if let Some(currency) = selected {
                        config.1.set_currency_id(&config.0.unwrap(), currency.id);
                        log::info!("Currency set to ID: {}", currency.id);
                    } else {
                        config.1.set_currency_id(&config.0.unwrap(), 0);
                        log::warn!("Currency not found in list, setting ID to 0.");
                    }
                }
                Err(e) => {
                    log::error!("Error retrieving currencies: {:?}", e);
                    config.1.set_currency_id(&config.0.unwrap(), 0);
                    log::warn!("Setting currency ID to 0 due to error.");
                }
            }
            *progress = 100.;
            log::info!("Import from JSON completed successfully.");
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to open file at URL: {:?}, error: {:?}", url, e);
            Err("Failed to open file.".to_string())
        }
    }
}
