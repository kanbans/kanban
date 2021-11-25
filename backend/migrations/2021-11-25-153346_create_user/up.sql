CREATE TABLE user (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);