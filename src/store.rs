use crate::{
    errors::DataStoreError,
    get_database_url,
    models::{self, Account, NewAccount},
    schema::{self, account, account_transfer, category, money_transaction},
};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use models::*;
use schema::account::dsl::*;
use schema::account_transfer::dsl::*;
use schema::category::dsl::*;
use schema::currency::dsl::*;
use schema::money_transaction::dsl::*;

pub struct Store {
    connection: SqliteConnection,
}

impl Default for Store {
    fn default() -> Self {
        let database_url = get_database_url();
        Self {
            connection: SqliteConnection::establish(database_url.to_str().unwrap()).unwrap_or_else(
                |_| panic!("Error connecting to {}", database_url.to_str().unwrap()),
            ),
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

    pub fn create_accounts(
        &mut self,
        new_accounts: &Vec<NewAccount>,
    ) -> Result<(), DataStoreError> {
        self.connection
            .transaction::<_, DieselError, _>(|conn| {
                for new_account in new_accounts {
                    diesel::insert_into(account::table)
                        .values(new_account)
                        .execute(conn)?;
                }
                Ok(())
            })
            .map_err(|e| DataStoreError::InsertError(e.to_string()))
    }

    pub fn update_transaction(
        &mut self,
        updated_transaction: &UpdateTransaction,
    ) -> Result<(), DataStoreError> {
        self.connection
            .transaction::<_, DieselError, _>(|conn| {
                diesel::update(money_transaction::table)
                    .filter(money_transaction::id.eq(updated_transaction.id))
                    .set((
                        money_transaction::bank_account.eq(updated_transaction.bank_account),
                        money_transaction::transaction_category
                            .eq(updated_transaction.transaction_category),
                        money_transaction::description.eq(&updated_transaction.description),
                        money_transaction::amount.eq(updated_transaction.amount),
                        money_transaction::transaction_date
                            .eq(updated_transaction.transaction_date),
                        money_transaction::is_expense.eq(updated_transaction.is_expense),
                    ))
                    .execute(conn)?;

                Ok(())
            })
            .map_err(|e| DataStoreError::UpdateError(e.to_string()))
    }

    pub fn delete_transaction(&mut self, transaction_id: &i32) -> Result<(), DataStoreError> {
        self.connection
            .transaction::<_, DieselError, _>(|conn| {
                diesel::delete(money_transaction::table)
                    .filter(money_transaction::id.eq(transaction_id))
                    .execute(conn)?;

                Ok(())
            })
            .map_err(|e| DataStoreError::DeleteError(e.to_string()))
    }

    pub fn update_account(&mut self, update_account: &UpdateAccount) -> Result<(), DataStoreError> {
        use schema::account::dsl::*;

        let res = diesel::update(account.filter(id.eq(update_account.id)))
            .set((
                name.eq(&update_account.name),
                account_description.eq(&update_account.account_description),
                initial_balance.eq(&update_account.initial_balance),
            ))
            .execute(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::UpdateError(e.to_string()));
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

    pub fn get_account_balance(&mut self, account_id: i32) -> Result<f32, DataStoreError> {
        // read account initial balance
        let target_initial_balance = account
            .filter(account::id.eq(account_id))
            .select(account::initial_balance)
            .first::<f32>(&mut self.connection);

        let mut total: f32 = target_initial_balance.unwrap_or(0.);

        // read all transaction on that account
        let transactions = money_transaction
            .filter(money_transaction::bank_account.eq(account_id))
            .select(MoneyTransaction::as_select())
            .load(&mut self.connection);

        if let Ok(transactions) = transactions {
            for t in transactions {
                if t.is_expense {
                    total -= t.amount;
                } else {
                    total += t.amount;
                }
            }
        }

        // read all transfer on that account
        let transfers = account_transfer
            .filter(
                account_transfer::from_account
                    .eq(account_id)
                    .or(account_transfer::to_account.eq(account_id)),
            )
            .select(AccountTransfer::as_select())
            .load(&mut self.connection);

        if let Ok(transfers) = transfers {
            for t in transfers {
                if t.from_account == account_id {
                    total -= t.amount;
                }
                if t.to_account == account_id {
                    total += t.amount;
                }
            }
        }

        Ok(total)
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

    pub fn calculate_expense_by_category(
        &mut self,
        category_id: i32,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<f32, DataStoreError> {
        use diesel::dsl::sum;
        use schema::money_transaction::dsl::*;

        let total_expense = money_transaction
            .filter(transaction_category.eq(category_id))
            .filter(
                transaction_date.between(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59)),
            )
            .filter(is_expense.eq(true))
            .select(sum(amount))
            .first::<Option<f32>>(&mut self.connection);

        match total_expense {
            Ok(Some(total)) => Ok(total),
            Ok(None) => Ok(0.0),
            Err(e) => Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn calculate_income_by_category(
        &mut self,
        category_id: i32,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<f32, DataStoreError> {
        use diesel::dsl::sum;
        use schema::money_transaction::dsl::*;

        let total_expense = money_transaction
            .filter(transaction_category.eq(category_id))
            .filter(
                transaction_date.between(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59)),
            )
            .filter(is_expense.eq(false))
            .select(sum(amount))
            .first::<Option<f32>>(&mut self.connection);

        match total_expense {
            Ok(Some(total)) => Ok(total),
            Ok(None) => Ok(0.0),
            Err(e) => Err(DataStoreError::QueryError(e.to_string())),
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

    pub fn create_categories(
        &mut self,
        new_categories: &Vec<NewCategory>,
    ) -> Result<(), DataStoreError> {
        self.connection
            .transaction::<_, DieselError, _>(|conn| {
                for new_category in new_categories {
                    diesel::insert_into(category::table)
                        .values(new_category)
                        .execute(conn)?;
                }
                Ok(())
            })
            .map_err(|e| DataStoreError::InsertError(e.to_string()))
    }

    pub fn update_category(
        &mut self,
        update_category: &UpdateCategory,
    ) -> Result<(), DataStoreError> {
        use schema::category::dsl::*;

        let res = diesel::update(category.filter(id.eq(update_category.id)))
            .set((
                name.eq(&update_category.name),
                category_description.eq(&update_category.category_description),
                is_income.eq(&update_category.is_income),
            ))
            .execute(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::UpdateError(e.to_string()));
        }

        Ok(())
    }

    pub fn get_money_transactions(&mut self) -> Result<Vec<MoneyTransaction>, DataStoreError> {
        let results = money_transaction
            .select(MoneyTransaction::as_select())
            .order(transaction_date.desc())
            .load(&mut self.connection);

        match results {
            Ok(results) => return Ok(results),
            Err(e) => return Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn get_money_transactions_date_range(
        &mut self,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<MoneyTransaction>, DataStoreError> {
        let results = money_transaction
            .filter(
                transaction_date.between(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59)),
            )
            .select(MoneyTransaction::as_select())
            .order(transaction_date.desc())
            .load(&mut self.connection);

        match results {
            Ok(results) => Ok(results),
            Err(e) => Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn create_money_transaction(
        &mut self,
        new_money_transaction: &NewMoneyTransaction,
    ) -> Result<(), DataStoreError> {
        let res = diesel::insert_into(money_transaction::table)
            .values(new_money_transaction)
            .returning(MoneyTransaction::as_returning())
            .get_result(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::InsertError(e.to_string()));
        }

        Ok(())
    }

    pub fn create_money_transactions(
        &mut self,
        new_money_transactions: &Vec<NewMoneyTransaction>,
    ) -> Result<(), DataStoreError> {
        let res = diesel::insert_into(money_transaction::table)
            .values(new_money_transactions)
            .execute(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::InsertError(e.to_string()));
        }

        Ok(())
    }

    pub fn get_currencies(&mut self) -> Result<Vec<Currency>, DataStoreError> {
        let results = currency
            .select(Currency::as_select())
            .load(&mut self.connection);

        match results {
            Ok(results) => return Ok(results),
            Err(e) => return Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn get_currency_symbol_by_id(
        &mut self,
        currency_id: i32,
    ) -> Result<String, DataStoreError> {
        use crate::schema::currency::dsl::{currency, id, symbol};

        let result = currency
            .filter(id.eq(currency_id))
            .select(symbol)
            .first::<String>(&mut self.connection);

        match result {
            Ok(currency_symbol) => Ok(currency_symbol),
            Err(e) => Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn get_transfers(&mut self) -> Result<Vec<AccountTransfer>, DataStoreError> {
        let results = account_transfer
            .select(AccountTransfer::as_select())
            .load(&mut self.connection);

        match results {
            Ok(results) => return Ok(results),
            Err(e) => return Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn get_transfers_date_range(
        &mut self,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<AccountTransfer>, DataStoreError> {
        let results = account_transfer
            .filter(
                transfer_date.between(start_date.and_hms(0, 0, 0), end_date.and_hms(23, 59, 59)),
            )
            .select(AccountTransfer::as_select())
            .order(transfer_date.desc())
            .load(&mut self.connection);

        match results {
            Ok(results) => Ok(results),
            Err(e) => Err(DataStoreError::QueryError(e.to_string())),
        }
    }

    pub fn create_account_transfer(
        &mut self,
        new_account_transfer: &NewAccountTransfer,
    ) -> Result<(), DataStoreError> {
        let res = diesel::insert_into(account_transfer::table)
            .values(new_account_transfer)
            .execute(&mut self.connection);

        if let Err(e) = res {
            return Err(DataStoreError::InsertError(e.to_string()));
        }

        Ok(())
    }

    pub fn drop_all(&mut self) -> Result<(), DataStoreError> {
        log::info!("Deleting all tables...");
        let _ = diesel::delete(account).execute(&mut self.connection);
        let _ = diesel::delete(money_transaction).execute(&mut self.connection);
        let _ = diesel::delete(category).execute(&mut self.connection);
        log::info!("All tables deleted.");
        Ok(())
    }
}
