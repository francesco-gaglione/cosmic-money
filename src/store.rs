use crate::{
    errors::DataStoreError,
    models::{self, Account, NewAccount},
    schema::{self, account, category},
};
use diesel::prelude::*;
use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use models::*;
use schema::account::dsl::*;
use schema::category::dsl::*;
use std::env;

pub struct Store {
    connection: SqliteConnection,
}

impl Default for Store {
    fn default() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            connection: SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        }
    }
}

impl Store {
    pub fn create_account(&mut self, new_account: &NewAccount) -> Result<(), DataStoreError> {
        let res = diesel::insert_into(account::table)
            .values(new_account)
            .returning(Account::as_returning())
            .get_result(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::InsertError(e.to_string()));
        }

        Ok(())
    }

    pub fn get_accounts(&mut self) -> Result<Vec<Account>, DataStoreError> {
        let results = account
            .select(Account::as_select())
            .load(&mut self.connection);

        match results {
            Ok(results) => return Ok(results),
            Err(e) => return Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn get_categories(&mut self) -> Result<Vec<Category>, DataStoreError> {
        let results = category
            .select(Category::as_select())
            .load(&mut self.connection);

        match results {
            Ok(results) => return Ok(results),
            Err(e) => return Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn create_category(&mut self, new_category: &NewCategory) -> Result<(), DataStoreError> {
        let res = diesel::insert_into(category::table)
            .values(new_category)
            .returning(Category::as_returning())
            .get_result(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::InsertError(e.to_string()));
        }

        Ok(())
    }
}
