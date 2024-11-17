// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        account_description -> Text,
        initial_balance -> Float,
    }
}

diesel::table! {
    account_transfer (id) {
        id -> Integer,
        from_account -> Integer,
        to_account -> Integer,
        transfer_date -> Timestamp,
        description -> Nullable<Text>,
        amount -> Float,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
        category_description -> Text,
        is_income -> Bool,
    }
}

diesel::table! {
    currency (id) {
        id -> Integer,
        label -> Text,
        symbol -> Text,
    }
}

diesel::table! {
    money_transaction (id) {
        id -> Integer,
        bank_account -> Integer,
        transaction_category -> Integer,
        description -> Text,
        amount -> Float,
        transaction_date -> Timestamp,
        is_expense -> Bool,
    }
}

diesel::joinable!(money_transaction -> account (bank_account));
diesel::joinable!(money_transaction -> category (transaction_category));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    account_transfer,
    category,
    currency,
    money_transaction,
);
