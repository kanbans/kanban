CREATE TABLE sessions (
  session_token VARCHAR(88) PRIMARY KEY NOT NULL,
  belongs_to VARCHAR(36) NOT NULL,
  FOREIGN KEY (belongs_to) REFERENCES users(id) ON DELETE CASCADE
);