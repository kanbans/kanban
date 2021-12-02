CREATE TABLE sessions (
  session_token VARCHAR(64) PRIMARY KEY NOT NULL,
  belongs_to VARCHAR(36) NOT NULL,
  FOREIGN KEY (belongs_to) REFERENCES user(id) ON DELETE CASCADE
);