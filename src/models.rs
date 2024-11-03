use crate::schema::account;
use crate::schema::category;
use crate::schema::money_transaction;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub initial_balance: f32,
    pub account_description: String,
}

impl AsRef<str> for Account {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub name: String,
    pub initial_balance: f32,
    pub account_description: String,
}

impl From<&Account> for NewAccount {
    fn from(value: &Account) -> Self {
        Self {
            name: value.name.clone(),
            initial_balance: value.initial_balance,
            account_description: value.account_description.clone(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct UpdateAccount {
    pub id: i32,
    pub name: String,
    pub initial_balance: f32,
    pub account_description: String,
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub category_description: String,
    pub is_income: bool,
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub name: String,
    pub is_income: bool,
    pub category_description: String,
}

impl From<&Category> for NewCategory {
    fn from(value: &Category) -> Self {
        Self {
            name: value.name.clone(),
            is_income: value.is_income,
            category_description: value.category_description.clone(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = category)]
pub struct UpdateCategory<'a> {
    pub id: i32,
    pub name: &'a str,
    pub is_income: bool,
    pub category_description: String,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::money_transaction)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MoneyTransaction {
    pub id: i32,
    pub bank_account: i32, // Foreign key referencing the 'account' table
    pub transaction_category: i32, // Foreign key referencing the 'category' table
    pub description: String,
    pub amount: f32,
    pub transaction_date: chrono::NaiveDateTime,
    pub is_expense: bool,
}

#[derive(Insertable)]
#[diesel(table_name = money_transaction)]
pub struct NewMoneyTransaction {
    pub bank_account: i32,
    pub transaction_category: i32,
    pub description: String,
    pub amount: f32,
    pub transaction_date: chrono::NaiveDateTime,
    pub is_expense: bool,
}

impl From<&MoneyTransaction> for NewMoneyTransaction {
    fn from(value: &MoneyTransaction) -> Self {
        Self {
            bank_account: value.bank_account,
            transaction_category: value.transaction_category,
            description: value.description.clone(),
            amount: value.amount,
            transaction_date: value.transaction_date,
            is_expense: value.is_expense,
        }
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::currency)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Currency {
    pub id: i32,
    pub label: String,
    pub symbol: String,
}

impl AsRef<str> for Currency {
    fn as_ref(&self) -> &str {
        &self.label
    }
}
