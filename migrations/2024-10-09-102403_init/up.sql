CREATE TABLE account (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  account_type VARCHAR NOT NULL,
  initial_balance REAL NOT NULL DEFAULT 0
);

CREATE TABLE category (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL
);

INSERT INTO account (name, account_type, initial_balance) 
VALUES ('Checking Account', 'Bank', 1000.00);

INSERT INTO category (name) VALUES ('Groceries');
INSERT INTO category (name) VALUES ('Salary');
INSERT INTO category (name) VALUES ('Investments');
INSERT INTO category (name) VALUES ('Entertainment');
