table! {
    esi_wallet_journals (id) {
        amount -> Nullable<Float>,
        balance -> Nullable<Float>,
        date -> Timestamp,
        description -> Text,
        id -> BigInt,
        ref_type -> Text,
        tax -> Nullable<Float>,
    }
}

table! {
    esi_wallet_transactions (transaction_id) {
        client_id -> Integer,
        date -> Timestamp,
        is_buy -> Bool,
        is_personal -> Bool,
        journal_ref_id -> BigInt,
        location_id -> BigInt,
        quantity -> Integer,
        transaction_id -> BigInt,
        type_id -> Integer,
        unit_price -> Float,
    }
}

table! {
    eve_characters (id) {
        id -> Integer,
        name -> Text,
        access_token -> Text,
        refresh_token -> Text,
        expiry_date -> Timestamp,
    }
}

joinable!(esi_wallet_transactions -> esi_wallet_journals (journal_ref_id));

allow_tables_to_appear_in_same_query!(
    esi_wallet_journals,
    esi_wallet_transactions,
    eve_characters,
);
