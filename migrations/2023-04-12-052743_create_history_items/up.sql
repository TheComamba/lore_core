CREATE TABLE history_items (
  timestamp BIGINT NOT NULL,
  year INTEGER NOT NULL,
  day INTEGER,
  content TEXT NOT NULL,
  properties TEXT,
  PRIMARY KEY (timestamp)
);