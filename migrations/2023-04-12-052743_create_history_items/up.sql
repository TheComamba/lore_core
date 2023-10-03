CREATE TABLE history_items (
  timestamp INTEGER NOT NULL,
  year INTEGER NOT NULL,
  day INTEGER,
  content TEXT NOT NULL,
  properties TEXT,
  PRIMARY KEY (timestamp)
);