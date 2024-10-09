// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        account_type -> Text,
        initial_balance -> Float,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account,
    category,
);
