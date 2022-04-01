--Add columns to raspberrytracker table to store user preferences for visible event list contents.
--Update new columns with default (all visible) status.
--Increase DB version 0.41

ALTER TABLE raspberrytracker ADD event_kills_death BOOLEAN;
ALTER TABLE raspberrytracker ADD event_experience BOOLEAN;
ALTER TABLE raspberrytracker ADD event_revives BOOLEAN;
ALTER TABLE raspberrytracker ADD event_vehicles BOOLEAN;
ALTER TABLE raspberrytracker ADD event_achievements BOOLEAN;

UPDATE raspberrytracker set event_kills_death = TRUE, event_experience = TRUE, event_revives = TRUE, event_vehicles = TRUE, event_achievements = TRUE, version = 0.41;
