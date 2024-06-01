CREATE TABLE history_items (
  timestamp BIGINT NOT NULL,
  year INTEGER NOT NULL,
  day INTEGER NOT NULL,
  content TEXT NOT NULL,
  properties TEXT NOT NULL,
  PRIMARY KEY (timestamp)
);