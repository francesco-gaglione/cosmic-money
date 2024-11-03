use std::{fs::File, io::Write};

use cosmic::dialog::ashpd::url::Url;

use crate::{config::Config, STORE};

use super::model::SyncModel;

pub fn export_to_folder(url: Url, progress: &mut f32) -> Result<(), String> {
    let mut store = STORE.lock().unwrap();
    let config = Config::load();
    let accounts = store.get_accounts();
    let categories = store.get_categories();
    let transactions = store.get_money_transactions();
    let currencies = store.get_currencies();

    *progress = 25.;

    let currency = if let Ok(currencies) = currencies {
        match currencies.iter().find(|c| c.id == config.1.currency_id) {
            Some(currency) => currency.symbol.clone(),
            None => "USD".to_string(),
        }
    } else {
        "USD".to_string()
    };

    *progress = 50.;

    let sync_model = SyncModel {
        accounts: accounts.unwrap_or(vec![]),
        categories: categories.unwrap_or(vec![]),
        transactions: transactions.unwrap_or(vec![]),
        currency,
    };

    *progress = 75.;

    match serde_json::to_string(&sync_model) {
        Ok(serialized) => {
            if let Ok(path) = url.to_file_path() {
                let target = path.join("exported.json");
                match File::create(&target)
                    .and_then(|mut file| file.write_all(serialized.as_bytes()))
                {
                    Ok(_) => {
                        log::info!("file exported");
                        *progress = 100.;
                        Ok(())
                    }
                    Err(_) => Err("Failed to create file".to_string()),
                }
            } else {
                Err("Failed to read destination folder".to_string())
            }
        }
        Err(_) => Err("Failed to serialize files".to_string()),
    }
}
