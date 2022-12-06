// @generated automatically by Diesel CLI.

diesel::table! {
    price (id) {
        id -> Integer,
        security_id -> Integer,
        date -> Text,
        time -> Nullable<Text>,
        value -> Integer,
        denom -> Integer,
        currency -> Text,
    }
}

diesel::table! {
    security (id) {
        id -> Integer,
        namespace -> Nullable<Text>,
        symbol -> Text,
        updater -> Nullable<Text>,
        currency -> Nullable<Text>,
        ledger_symbol -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    price,
    security,
);

diesel::joinable!(price -> security (security_id));
