// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        account_type -> Text,
    }
}
