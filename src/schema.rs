table! {
    complete_transactions (buy_transaction_id, sell_transaction_id) {
        character_id -> Integer,
        buy_transaction_id -> Bigint,
        sell_transaction_id -> Bigint,
        bought_unit_price -> Nullable<Float>,
        bought_unit_taxes -> Nullable<Float>,
        sold_unit_price -> Float,
        sold_unit_taxes -> Float,
        amount -> Integer,
    }
}

table! {
    eve_characters (id) {
        id -> Integer,
        name -> Varchar,
        access_token -> Varchar,
        refresh_token -> Varchar,
        expiry_date -> Datetime,
        last_update -> Datetime,
        sell_tax -> Float,
        broker_fee -> Float,
    }
}

table! {
    inv_types (type_id) {
        type_id -> Integer,
        type_name -> Text,
    }
}

table! {
    transaction_queues (character_id, transaction_id) {
        character_id -> Integer,
        type_id -> Integer,
        transaction_id -> Bigint,
        amount_left -> Integer,
    }
}

table! {
    wallet_transactions (transaction_id) {
        transaction_id -> Bigint,
        character_id -> Integer,
        client_id -> Integer,
        date -> Datetime,
        is_buy -> Bool,
        is_personal -> Bool,
        location_id -> Bigint,
        quantity -> Integer,
        type_id -> Integer,
        unit_price -> Float,
        unit_taxes -> Float,
    }
}

joinable!(complete_transactions -> eve_characters (character_id));
joinable!(transaction_queues -> eve_characters (character_id));
joinable!(transaction_queues -> inv_types (type_id));
joinable!(transaction_queues -> wallet_transactions (transaction_id));
joinable!(wallet_transactions -> eve_characters (character_id));
joinable!(wallet_transactions -> inv_types (type_id));

allow_tables_to_appear_in_same_query!(
    complete_transactions,
    eve_characters,
    inv_types,
    transaction_queues,
    wallet_transactions,
);
