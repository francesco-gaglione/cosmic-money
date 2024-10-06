use crate::{
    models::{self, Account, NewAccount},
    schema::{self, account},
};
use diesel::prelude::*;
use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use models::*;
use schema::account::dsl::*;
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
    pub fn create_account(&mut self) -> Result<(), String> {
        let new_account = NewAccount {
            name: "test",
            account_type: "test",
        };

        let res = diesel::insert_into(account::table)
            .values(&new_account)
            .returning(Account::as_returning())
            .get_result(&mut self.connection)
            .expect("Error saving new post");

        Ok(())
    }

    pub fn get_accounts(&mut self) -> Result<Vec<Account>, String> {
        let results = account
            .select(Account::as_select())
            .load(&mut self.connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.name);
            println!("-----------\n");
            println!("{}", post.account_type);
        }

        Ok(Vec::new())
    }
}
