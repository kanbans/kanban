CREATE TABLE card (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  codename VARCHAR NOT NULL UNIQUE,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  priority INTEGER NOT NULL,
  column VARCHAR(36) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  assigned_to VARCHAR(36),
  tags_json VARCHAR,
  FOREIGN KEY (column) REFERENCES column(id),
  FOREIGN KEY (created_by) REFERENCES user(id),
  FOREIGN KEY (assigned_to) REFERENCES user(id)
);