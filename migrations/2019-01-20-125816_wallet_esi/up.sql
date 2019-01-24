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
    unit_price REAL NOT NULL,
    unit_taxes REAL NOT NULL,
    FOREIGN KEY (character_id) REFERENCES eve_characters(id)
);

CREATE TABLE transaction_queues (
    character_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    transaction_id BIGINT NOT NULL,
    amount_left INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (transaction_id) REFERENCES wallet_transactions(transaction_id),
    PRIMARY KEY (character_id, transaction_id)
);

CREATE TABLE complete_transactions (
    character_id INTEGER NOT NULL,
    buy_transaction_id BIGINT,
    sell_transaction_id BIGINT NOT NULL,
    bought_unit_price REAL,
    bought_unit_taxes REAL,
    sold_unit_price REAL NOT NULL,
    sold_unit_taxes REAL NOT NULL,
    amount INTEGER NOT NULL,    
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (buy_transaction_id) REFERENCES wallet_transactions(transaction_id),
    FOREIGN KEY (sell_transaction_id) REFERENCES wallet_transactions(transaction_id),
    PRIMARY KEY (buy_transaction_id, sell_transaction_id)
);