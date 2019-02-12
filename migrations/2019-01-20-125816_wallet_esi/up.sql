CREATE TABLE wallet_transactions (    
    transaction_id BIGINT PRIMARY KEY NOT NULL,
    character_id INTEGER NOT NULL,
    client_id INTEGER NOT NULL,    
    date DATETIME NOT NULL,
    is_buy BOOLEAN NOT NULL,
    is_personal BOOLEAN NOT NULL,
    location_id BIGINT NOT NULL,
    quantity INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    unit_price FLOAT NOT NULL,
    unit_taxes FLOAT NOT NULL,
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (type_id) REFERENCES inv_types(type_id)
);

CREATE TABLE transaction_queues (
    character_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    transaction_id BIGINT NOT NULL,
    amount_left INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (transaction_id) REFERENCES wallet_transactions(transaction_id),
    PRIMARY KEY (character_id, transaction_id),
    FOREIGN KEY (type_id) REFERENCES inv_types(type_id)
);

CREATE TABLE complete_transactions (
    character_id INTEGER NOT NULL,
    buy_transaction_id BIGINT,
    sell_transaction_id BIGINT NOT NULL,
    bought_unit_price FLOAT,
    bought_unit_taxes FLOAT,
    sold_unit_price FLOAT NOT NULL,
    sold_unit_taxes FLOAT NOT NULL,
    amount INTEGER NOT NULL,    
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (buy_transaction_id) REFERENCES wallet_transactions(transaction_id),
    FOREIGN KEY (sell_transaction_id) REFERENCES wallet_transactions(transaction_id),
    PRIMARY KEY (buy_transaction_id, sell_transaction_id)
);

CREATE VIEW complete_transactions_views (
    type_name,
    type_id,
    character_id,
    sell_transaction_id,
    buy_transaction_id,
    sell_date,
    buy_date,
    is_buy,
    quantity,
    buy_unit_price,
    buy_unit_tax,
    sell_unit_price,
    sell_unit_tax
) AS 
    SELECT
        it.type_name,
        it.type_id,
        wtsell.character_id,
        wtsell.transaction_id,
        wtbuy.transaction_id,
        wtsell.date,
        wtbuy.date,
        FALSE,
        ct.amount,
        wtbuy.unit_price,
        wtbuy.unit_taxes,
        wtsell.unit_price,
        wtsell.unit_taxes
    FROM complete_transactions AS ct
    LEFT JOIN wallet_transactions AS wtsell ON wtsell.transaction_id = ct.sell_transaction_id
    LEFT JOIN wallet_transactions AS wtbuy ON wtbuy.transaction_id = ct.buy_transaction_id
    LEFT JOIN inv_types AS it ON it.type_id = wtsell.type_id
UNION
    SELECT
        it.type_name,
        it.type_id,
        wt.character_id,
        wt.transaction_id,
        NULL,
        wt.date,
        NULL,
        wt.is_buy,
        wt.quantity,
        NULL,
        NULL,
        wt.unit_price,
        wt.unit_taxes
    FROM wallet_transactions as wt
    LEFT JOIN inv_types AS it ON it.type_id = wt.type_id
    WHERE wt.is_buy = TRUE;




