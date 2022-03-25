--Version 0.3, Add tables to support storing session histories


CREATE TABLE IF NOT EXISTS sessions
(
  id INTEGER NOT NULL,

  c_full_name TEXT,
  c_lower_name TEXT,
  c_server INTEGER,
  c_outfit TEXT,
  c_outfit_full TEXT,
  c_id TEXT,
  c_faction INTEGER,
  c_br INTEGER,
  c_asp INTEGER,

  start_time INTEGER,
  end_time INTEGER,

  kill_count INTEGER,
  death_count INTEGER,
  headshot_kills INTEGER,
  headshot_deaths INTEGER,
  vehicles_destroyed INTEGER,
  vehicles_lost INTEGER,
  vehicle_kills INTEGER,
  vehicle_deaths INTEGER,
  time_zone TEXT, 
  
  i_kills INTEGER,
  i_actual_deaths INTEGER,
  i_revive_deaths INTEGER,
  i_destroyed INTEGER,
  i_fired INTEGER,
  i_hits INTEGER,
  i_headshots INTEGER,

  la_kills INTEGER,
  la_revived_deaths INTEGER,
  la_fired INTEGER,
  la_shots INTEGER,
  la_headshots INTEGER,

  l_br INTEGER,
  l_asp INTEGER,
  pa_rankups INTEGER,

  PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS events
(
  session INTEGER NOT NULL,

  ordering INTEGER NOT NULL,

  kind INTEGER,
  faction INTEGER,
  br INTEGER,
  asp INTEGER,
  class INTEGER,
  name TEXT,
  weapon TEXT,
  weapon_id TEXT,
  headshot BOOLEAN,
  kdr REAL,
  time_stamp INTEGER,
  vehicle INTEGER,
  date_time TEXT,

  FOREIGN KEY(session) REFERENCES sessions(id)
);

CREATE TABLE IF NOT EXISTS weaponstats
(
  session INTEGER NOT NULL,

  ordering INTEGER NOT NULL,

  weapon_id TEXT,
  name TEXT,
  kills INTEGER,
  headshots INTEGER,
  hits INTEGER,
  fired INTEGER,

  i_fired INTEGER,
  i_hits INTEGER,
  i_kills INTEGER,
  i_headshots INTEGER,

  FOREIGN KEY(session) REFERENCES sessions(id)
);

UPDATE raspberrytracker SET version = 0.3; 
