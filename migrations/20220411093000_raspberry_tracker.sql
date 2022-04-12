--Add tables to support storing sounds to be played when earning achievements.
--Increase DB version to 0.5

CREATE TABLE soundsets
(
  id INTEGER NOT NULL,
  name TEXT,
  filename TEXT,
  author TEXT,
  description TEXT,
  PRIMARY KEY (id)
);

CREATE TABLE sounds
(
  parent_set INTEGER NOT NULL,
  filename TEXT,
  achievement TEXT,
  audio BLOB,

  FOREIGN KEY(parent_set) REFERENCES soundsets(id)
);

ALTER TABLE raspberrytracker ADD current_soundset TEXT; --may be Null when no set selected

UPDATE raspberrytracker SET version = 0.5; 
