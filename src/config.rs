use std::any::TypeId;

use cosmic::Application;
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry},
    iced::Subscription,
};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub is_user_initialized: bool,
    pub currency_id: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_user_initialized: false,
            currency_id: 1,
        }
    }
}

impl Config {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new(crate::app::MoneyManager::APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Config::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading config: {:?}", errs);
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create config handler: {}", err);
                (None, Config::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct ConfigSubscription;
        cosmic_config::config_subscription(
            TypeId::of::<ConfigSubscription>(),
            crate::app::MoneyManager::APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}
