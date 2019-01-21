table! {
    eve_characters (id) {
        id -> Integer,
        name -> Text,
        access_token -> Text,
        refresh_token -> Text,
        expiry_date -> Timestamp,
    }
}

table! {
    wallet_transactions (transaction_id) {
        transaction_id -> Integer,
        client_id -> Integer,
        journal_id -> BigInt,
        date -> Timestamp,
        is_buy -> Bool,
        is_personal -> Bool,
        location_id -> BigInt,
        quantity -> Integer,
        type_id -> Integer,
        unit_price -> Float,
        description -> Text,
        total_amount -> Float,
        tax -> Float,
    }
}

allow_tables_to_appear_in_same_query!(
    eve_characters,
    wallet_transactions,
);
