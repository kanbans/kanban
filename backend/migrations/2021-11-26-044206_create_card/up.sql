CREATE TABLE cards (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  codename VARCHAR NOT NULL UNIQUE,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  priority INTEGER NOT NULL,
  column VARCHAR(36) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  assigned_to VARCHAR(36),
  FOREIGN KEY (column) REFERENCES columns(id) ON DELETE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (assigned_to) REFERENCES users(id)
);