CREATE TABLE account (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  account_type VARCHAR NOT NULL,
  account_description VARCHAR NOT NULL,
  initial_balance REAL NOT NULL DEFAULT 0
);

CREATE TABLE category (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  category_description VARCHAR NOT NULL,
  is_income BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE money_transaction (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  bank_account INTEGER NOT NULL,
  transaction_category INTEGER NOT NULL,
  description VARCHAR NOT NULL,
  amount REAL NOT NULL,
  transaction_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  is_expense BOOLEAN DEFAULT TRUE NOT NULL,
  FOREIGN KEY (bank_account) REFERENCES account(id),
  FOREIGN KEY (transaction_category) REFERENCES category(id)
);

INSERT INTO account (name, account_type, initial_balance, account_description)
VALUES ('Checking Account', 'Bank', 1000.00, "account description");


INSERT INTO category (name, category_description, is_income)
VALUES ('Groceries', 'Expenses for food and household supplies', FALSE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Salary', 'Income from employment', TRUE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Investments', 'Income or expenses related to investments', FALSE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Entertainment', 'Expenses for leisure activities', FALSE);
