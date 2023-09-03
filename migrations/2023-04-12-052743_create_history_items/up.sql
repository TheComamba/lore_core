CREATE TABLE history_items (
  label TEXT NOT NULL,
  year INTEGER NOT NULL,
  day INTEGER,
  content TEXT NOT NULL,
  properties TEXT,
  PRIMARY KEY (label)
);