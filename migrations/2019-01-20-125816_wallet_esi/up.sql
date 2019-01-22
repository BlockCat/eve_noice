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
    FOREIGN KEY (character_id) REFERENCES eve_characters(id)
);