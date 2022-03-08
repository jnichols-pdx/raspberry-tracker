--Version 0.2, 
CREATE TABLE images
(
  name TEXT NOT NULL,
  census_id INTEGER,
  img BLOB NOT NULL, 
  PRIMARY KEY (name)
);

UPDATE raspberrytracker SET version = 0.2; 
