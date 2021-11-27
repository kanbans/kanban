CREATE TABLE card_tags (
  card_id VARCHAR(36) PRIMARY KEY,
  tag_id VARCHAR(36) PRIMARY KEY,
  FOREIGN KEY (card_id) REFERENCES card(id),
  FOREIGN KEY (tag_id) REFERENCES tag(id)
);