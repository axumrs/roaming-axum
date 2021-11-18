CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    balance INTEGER NOT NULL DEFAULT 0,
    UNIQUE(username)
);
INSERT INTO
    account(username, balance)
VALUES
    ('axum.rs', 999999), 
    ('foo', 0), 
    ('bar', 0); 
