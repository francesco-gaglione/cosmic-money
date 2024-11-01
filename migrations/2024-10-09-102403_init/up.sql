CREATE TABLE account (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
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

CREATE TABLE currency (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  label VARCHAR NOT NULL,
  symbol VARCHAR(3) NOT NULL
);

INSERT INTO account (name, initial_balance, account_description)
VALUES ('Checking Account',  1000.00, "account description");

INSERT INTO category (name, category_description, is_income)
VALUES ('Groceries', 'Expenses for food and household supplies', FALSE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Salary', 'Income from employment', TRUE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Investments', 'Income or expenses related to investments', FALSE);

INSERT INTO category (name, category_description, is_income)
VALUES ('Entertainment', 'Expenses for leisure activities', FALSE);

INSERT INTO currency (label, symbol) VALUES
  ('US Dollar', 'USD'),
  ('Euro', 'EUR'),
  ('Japanese Yen', 'JPY'),
  ('British Pound', 'GBP'),
  ('Australian Dollar', 'AUD'),
  ('Canadian Dollar', 'CAD'),
  ('Swiss Franc', 'CHF'),
  ('Chinese Yuan', 'CNY'),
  ('Swedish Krona', 'SEK'),
  ('New Zealand Dollar', 'NZD'),
  ('Mexican Peso', 'MXN'),
  ('Singapore Dollar', 'SGD'),
  ('Hong Kong Dollar', 'HKD'),
  ('Norwegian Krone', 'NOK'),
  ('South Korean Won', 'KRW'),
  ('Turkish Lira', 'TRY'),
  ('Russian Ruble', 'RUB'),
  ('Indian Rupee', 'INR'),
  ('Brazilian Real', 'BRL'),
  ('South African Rand', 'ZAR'),
  ('Philippine Peso', 'PHP'),
  ('Czech Koruna', 'CZK'),
  ('Indonesian Rupiah', 'IDR'),
  ('Malaysian Ringgit', 'MYR'),
  ('Hungarian Forint', 'HUF'),
  ('Icelandic Krona', 'ISK'),
  ('Polish Zloty', 'PLN'),
  ('Thai Baht', 'THB'),
  ('Ukrainian Hryvnia', 'UAH'),
  ('Israeli New Shekel', 'ILS'),
  ('Chilean Peso', 'CLP'),
  ('Romanian Leu', 'RON'),
  ('Danish Krone', 'DKK'),
  ('New Taiwan Dollar', 'TWD'),
  ('Pakistani Rupee', 'PKR'),
  ('Argentine Peso', 'ARS'),
  ('Colombian Peso', 'COP'),
  ('Vietnamese Dong', 'VND'),
  ('Bangladeshi Taka', 'BDT'),
  ('Egyptian Pound', 'EGP');
