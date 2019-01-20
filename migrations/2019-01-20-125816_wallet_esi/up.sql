CREATE TABLE esi_wallet_journals (
    amount REAL,
    balance REAL,
    date DATETIME NOT NULL,
    description TEXT NOT NULL,
    id BigInt  PRIMARY KEY NOT NULL,
    ref_type TEXT NOT NULL,
    tax REAL
);

CREATE TABLE esi_wallet_transactions (
    client_id INTEGER NOT NULL,
    date DATETIME NOT NULL,
    is_buy BOOLEAN NOT NULL,
    is_personal BOOLEAN NOT NULL,
    journal_ref_id BigInt  NOT NULL,
    location_id BigInt  NOT NULL,
    quantity INTEGER NOT NULL,
    transaction_id BigInt  PRIMARY KEY NOT NULL,
    type_id INTEGER  NOT NULL,
    unit_price REAL NOT NULL,
    FOREIGN KEY(journal_ref_id) REFERENCES esi_wallet_journals(id)
);