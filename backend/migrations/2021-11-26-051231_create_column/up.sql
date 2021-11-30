CREATE TABLE column (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  belongs_to VARCHAR(36) NOT NULL,
  FOREIGN KEY (belongs_to) REFERENCES board(id)
);