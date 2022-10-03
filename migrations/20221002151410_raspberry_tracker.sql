--Add columns for tracking score (raw XP before multipliers) and times a player was revived 
--per session. Fill in these values for previous sessions from logged events.

--Increase DB version to 0.62

ALTER TABLE sessions ADD revived_count INTEGER;
ALTER TABLE sessions ADD session_score INTEGER;

--Fill in revived_count from previously recorded Revived events.
UPDATE sessions SET revived_count = revs FROM (SELECT s.id, COUNT(events.ordering) AS revs FROM events, sessions s ON events.session = s.id WHERE events.kind = 12 GROUP BY s.id) x WHERE x.id = sessions.id;

--Fill in session_score (XP) earned from previously recorded ExperienceTick events.
UPDATE sessions SET session_score = score FROM (SELECT s.id, SUM(weapon) AS score FROM events, sessions s ON events.session = s.id WHERE events.kind = 10 GROUP BY s.id) x WHERE x.id = sessions.id;

UPDATE raspberrytracker SET version = 0.62;
