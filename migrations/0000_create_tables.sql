CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  password_hash VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
  id SERIAL PRIMARY KEY,
  owner_id INTEGER NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  balance DECIMAL(10, 2) NOT NULL,
  FOREIGN KEY (owner_id) REFERENCES users(id)
);