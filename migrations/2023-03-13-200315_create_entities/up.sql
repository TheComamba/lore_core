CREATE TABLE entities (
  label TEXT NOT NULL,
  descriptor TEXT NOT NULL,
  description TEXT,
  PRIMARY KEY (label, descriptor)
);