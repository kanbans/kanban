CREATE TABLE card_tags (
  card_id VARCHAR(36) PRIMARY KEY NOT NULL,
  tag_id VARCHAR(36) NOT NULL,
  FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);