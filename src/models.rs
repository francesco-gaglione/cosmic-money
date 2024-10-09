use crate::schema::account;
use crate::schema::category;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub account_type: String,
    pub initial_balance: f32,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub account_type: &'a str,
    pub initial_balance: f32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory<'a> {
    pub name: &'a str,
}
