-- Your SQL goes here
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
    amount INTEGER NOT NULL,
    taxes REAL NOT NULL,    
    FOREIGN KEY (character_id) REFERENCES eve_characters(id),
    FOREIGN KEY (buy_transaction_id) REFERENCES wallet_transactions(transaction_id),
    FOREIGN KEY (sell_transaction_id) REFERENCES wallet_transactions(transaction_id),
    PRIMARY KEY (buy_transaction_id, sell_transaction_id)
);