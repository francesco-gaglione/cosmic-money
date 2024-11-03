use serde::{Deserialize, Serialize};

use crate::models::{Account, Category, MoneyTransaction};

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncModel {
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub transactions: Vec<MoneyTransaction>,
    pub currency: String, 
}

impl Default for SyncModel {
    fn default() -> Self {
        Self {
            accounts: vec![],
            categories: vec![],
            transactions: vec![],
            currency: "".to_string(),
        }
    }
}
