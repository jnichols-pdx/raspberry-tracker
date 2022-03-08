--Version 0.1, initial content
CREATE TABLE IF NOT EXISTS raspberrytracker 
(
  version REAL
);

INSERT INTO raspberrytracker VALUES (0.1); 

CREATE TABLE windows
(
  name TEXT,
  width REAL,
  height REAL 
);
INSERT INTO windows VALUES ('main', 800.0, 480.0);

CREATE TABLE IF NOT EXISTS characters
(
  name TEXT,
  lower_name TEXT,
  outfit TEXT,
  outfit_full TEXT,
  id TEXT NOT NULL,
  auto_track INTEGER,
  server INTEGER, 
  faction INTEGER,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS weapons
(
  id TEXT NOT NULL,
  name TEXT NOT NULL,
  PRIMARY KEY (id)
);
