CREATE TABLE wallet_transactions (
    transaction_id INTEGER PRIMARY KEY NOT NULL,
    client_id INTEGER NOT NULL,
    journal_id BIGINT NOT NULL,
    date DATETIME NOT NULL,
    is_buy BOOLEAN NOT NULL,
    is_personal BOOLEAN NOT NULL,
    location_id BIGINT NOT NULL,
    quantity INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    description TEXT NOT NULL,
    total_amount REAL NOT NULL,
    tax REAL NOT NULL
);